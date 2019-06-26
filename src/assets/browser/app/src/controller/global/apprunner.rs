use std::sync::{ Arc, Mutex, Weak };

use stdweb::web::HtmlElement;
use url::Url;

use composit::register_compositor_ticks;
use controller::global::{ App, GlobalWeak };
use controller::scheduler::{ Scheduler, SchedRun, SchedulerGroup };
use controller::input::{
    register_direct_events, register_user_events, register_dom_events
};
use controller::output::{ OutputAction, Report, ViewportReport, ZMenuReports };

#[cfg(any(not(deploy),console))]
use data::blackbox::{
    blackbox_report, blackbox_push, blackbox_pop, blackbox_tick
};

use data::{ HttpManager, BackendConfig };
use data::blackbox::BlackBoxDriver;
use dom::Bling;
use dom::event::EventControl;
use dom::domutil::browser_time;
use tácode::Tácode;

struct AppRunnerImpl {
    g: GlobalWeak,
    el: HtmlElement,
    bling: Box<Bling>,
    app: Arc<Mutex<App>>,
    controls: Vec<Box<EventControl<()>>>,
    sched_group: SchedulerGroup,
    tc: Tácode,
    debug_reporter: BlackBoxDriver,
    browser_el: HtmlElement
}

impl AppRunnerImpl {
    fn clear_controls(&mut self) {
        let controls = &mut self.controls;
        for control in &mut controls.iter_mut() {
            control.reset();
        }
        controls.clear();
    }
}

#[derive(Clone)]
pub struct AppRunner(Arc<Mutex<AppRunnerImpl>>);

#[derive(Clone)]
pub struct AppRunnerWeak(Weak<Mutex<AppRunnerImpl>>);

impl AppRunner {
    pub fn new(g: &GlobalWeak, http_manager: &HttpManager, el: &HtmlElement, bling: Box<Bling>, config_url: &Url, config: &BackendConfig, debug_reporter: BlackBoxDriver) -> AppRunner {
        let browser_el : HtmlElement = bling.apply_bling(&el);
        let tc = Tácode::new();
        let st = App::new(&tc,config,&http_manager,&browser_el,&config_url);
        let sched_group = {
            let g = unwrap!(g.clone().upgrade()).clone();
            g.scheduler().make_group()
        };
        let mut out = AppRunner(Arc::new(Mutex::new(AppRunnerImpl {
            g: g.clone(),
            el: el.clone(),
            bling,
            app: Arc::new(Mutex::new(st)),
            controls: Vec::<Box<EventControl<()>>>::new(),
            sched_group,
            tc: tc.clone(),
            debug_reporter,
            browser_el: browser_el.clone()
        })));
        {
            let imp = out.0.lock().unwrap();
            let weak = AppRunnerWeak(Arc::downgrade(&out.0));
            imp.app.lock().unwrap().set_runner(&weak);
        }
        out.init();
        let report = Report::new(&mut out);
        let viewport_report = ViewportReport::new(&mut out);
        let zmenu_reports = ZMenuReports::new(&mut out);
        {
            let mut imp = out.0.lock().unwrap();
            let app = imp.app.clone();
            app.lock().unwrap().set_report(report);
            app.lock().unwrap().set_viewport_report(viewport_report);
            app.lock().unwrap().set_zmenu_reports(zmenu_reports);
            let el = imp.el.clone();
            imp.bling.activate(&app,&el);
        }
        out
    }

    pub fn get_browser_el(&mut self) -> HtmlElement {
        self.0.lock().unwrap().browser_el.clone()
    }

    pub fn add_timer<F>(&mut self, name: &str, mut cb: F, prio: usize)
                            where F: FnMut(&mut App, f64, &mut SchedRun) -> Vec<OutputAction> + 'static {
        let mut ar = self.clone();
        let mut imp = self.0.lock().unwrap();
        let app = imp.app.clone();
        imp.sched_group.add(name,Box::new(move |sr| {
            let oas = cb(&mut app.lock().unwrap(),browser_time(),sr);
            for oa in oas {
                oa.run(&mut ar);
            }
        }),prio,false);
    }

    pub fn scheduler(&self) -> Scheduler {
        let g = unwrap!(unwrap!(self.0.lock()).g.upgrade()).clone();
        g.scheduler()
    }
    
    pub fn init(&mut self) {
        /* register main heartbeat of compositor */
        register_compositor_ticks(self);
        
        /* register canvas-bound events */
        {
            let el = self.0.lock().unwrap().el.clone();
            register_user_events(self,&el);
            register_direct_events(self,&el);
            register_dom_events(self,&el);
        }

        {
            {
                let mut imp = self.0.lock().unwrap();
                /* tacode */
                {
                    let tc = imp.tc.clone();
                    imp.sched_group.add("tácode",Box::new(move |sr| {
                        tc.step(sr.available());
                    }),2,false);
                }
                /* blackbox */
                #[cfg(any(not(deploy),console))]
                {
                    let mut dr = imp.debug_reporter.clone();
                    imp.sched_group.add("blackbox",Box::new(move |sr| {
                        if !blackbox_tick(&mut dr) {
                            sr.unproductive();
                        }
                    }),5,false);
                }
                /* draw */
                let app = imp.app.clone();
                imp.sched_group.add("draw",Box::new(move |_| {
                    app.lock().unwrap().draw();
                }),0,true);
            }
            /* xfer */
            self.add_timer("xfer",move |app,_,sr| {
                if !app.tick_xfer() {
                    sr.unproductive();
                }
                vec![]
            },2);
            /* resize check */
            self.add_timer("resizer",move |app,_,_| {
                app.check_size();
                vec![]
            },0);
            self.add_timer("gone-check",move |app,_,_| {
                if app.check_gone() {
                    vec![OutputAction::Destroy]
                } else {
                    vec![]
                }
            },0);
        }
        bb_log!("main","debug reporter initialised");
    }
        
    pub fn add_control(&mut self, control: Box<EventControl<()>>) {
        self.0.lock().unwrap().controls.push(control);
    }
    
    pub fn state(&self) -> Arc<Mutex<App>> {
        unwrap!(self.0.lock()).app.clone()
    }
    
    pub fn destroy(&mut self) {
        self.0.lock().unwrap().clear_controls();
        let r = self.state();
        r.lock().unwrap().destroy();
    }
        
    pub fn bling_key(&mut self, key: &str) {
        let mut imp = self.0.lock().unwrap();   
        let app = imp.app.clone();     
        imp.bling.key(&app,key);
    }
}

impl AppRunnerWeak {
    pub fn new(g : &AppRunner) -> AppRunnerWeak {
        AppRunnerWeak(Arc::downgrade(&g.0))
    }

    
    pub fn upgrade(&self) -> Option<AppRunner> {
        self.0.upgrade().map(|cr| AppRunner(cr))
    }
    
    pub fn none() -> AppRunnerWeak { AppRunnerWeak(Weak::new()) }
}

impl Drop for AppRunner {
    fn drop(&mut self) {
        console!("App runner dropped");
    }
}
