use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::{ Rc, Weak };
use std::sync::{ Arc, Mutex };

use stdweb::unstable::TryInto;
use stdweb::web::{ HtmlElement, Element, IHtmlElement, window };
use url::Url;
use util::set_instance_id;

use controller::input::{
    register_startup_events, register_shutdown_events
};
use controller::global::{ AppRunner, Booting };
use controller::scheduler::{ Scheduler, SchedulerGroup };
use data::{ BackendConfigBootstrap, HttpManager };
use dom::domutil;
use dom::domutil::browser_time;

const SCHEDULER_ALLOC : f64 = 12.; /* ms per raf */

pub struct GlobalImpl {
    apps: HashMap<String,AppRunner>,
    http_manager: HttpManager,
    scheduler: Scheduler,
    sched_group: SchedulerGroup
}

impl GlobalImpl {
    pub fn new() -> GlobalImpl {
        let scheduler = Scheduler::new();
        let sched_group = scheduler.make_group();
        let mut out = GlobalImpl {
            apps: HashMap::<String,AppRunner>::new(),
            http_manager: HttpManager::new(),
            scheduler,
            sched_group
        };
        out.init();
        out
    }

    fn init(&mut self) {
        self.scheduler.set_timesig(2);
        let http_manager = self.http_manager.clone();
        self.sched_group.add("http-manager",Box::new(move |sr| {
            if !http_manager.tick() {
                sr.unproductive();
            }
        }),3,false);
    }

    pub fn scheduler_clone(&self) -> Scheduler {
        self.scheduler.clone()
    }

    pub fn destroy(&mut self) {
        for app in self.apps.values_mut() {
            app.unregister();
        }
    }

    pub fn unregister_app(&mut self, key: &str) {
        if let Entry::Occupied(mut e) = self.apps.entry(key.to_string()) {
            e.get_mut().unregister();
        }
    }

    pub fn register_app(&mut self, key: &str, ar: AppRunner) {
        self.apps.insert(key.to_string(),ar);
    }
        
    pub fn with_apprunner<F,G>(&mut self, key: &str, cb:F) -> Option<G>
            where F: FnOnce(&mut AppRunner) -> G {
        if let Entry::Occupied(mut e) = self.apps.entry(key.to_string()) {
            let mut ar = e.get_mut().clone();
            Some(cb(&mut ar))
        } else {
            None
        }
    }    

    pub fn get_apprunner(&mut self, key: &str) -> Option<AppRunner> {
        if let Entry::Occupied(mut e) = self.apps.entry(key.to_string()) {
            let mut ar = e.get_mut().clone();
            Some(ar)
        } else {
            None
        }
    }    
}

#[derive(Clone)]
pub struct Global(Rc<RefCell<GlobalImpl>>);

#[derive(Clone)]
pub struct GlobalWeak(Weak<RefCell<GlobalImpl>>);

impl Global {
    pub fn new() -> Global {
        let mut out = Global(Rc::new(RefCell::new(GlobalImpl::new())));
        out.tick();
        out
    }
    
    pub fn tick(&mut self) {
        let sched = self.0.borrow_mut().scheduler_clone();
        sched.beat(SCHEDULER_ALLOC);
        let mut out = self.clone();
        window().request_animation_frame(
            move |t| out.tick()
        );
    }
    
    pub fn destroy(&mut self) {
        self.0.borrow_mut().destroy();
    }
    
    pub fn unregister_app(&mut self, key: &str) {
        self.0.borrow_mut().unregister_app(key);
    }

    pub fn register_app(&mut self, key: &str, el: &HtmlElement, debug: bool, config_url: &Url) {
        self.unregister_app(key);
        let http_manager = &self.0.borrow().http_manager.clone();
        let mut bcb = BackendConfigBootstrap::new(&http_manager.clone(),config_url);
        console!("preparing to boot");
        let b : Rc<RefCell<Booting>> = Rc::new(
            RefCell::new(
                Booting::new(self,http_manager,config_url,el,key,debug)
            )
        );
        bcb.add_callback(Box::new(move |config| {
            b.borrow_mut().boot(config);
        }));
    }
    
    pub fn register_app_now(&mut self, key: &str, ar: AppRunner) {
        self.0.borrow_mut().register_app(key,ar);
    }
    
    pub fn scheduler_clone(&self) -> Scheduler {
        self.0.borrow().scheduler_clone()
    }
    
    #[allow(unused,dead_code)]
    pub fn with_apprunner<F,G>(&mut self, key: &str, cb:F) -> Option<G>
            where F: FnOnce(&mut AppRunner) -> G {
        self.0.borrow_mut().with_apprunner(key,cb)
    }
}

impl GlobalWeak {
    pub fn new(g : &Global) -> GlobalWeak {
        GlobalWeak(Rc::downgrade(&g.0))
    }
    
    pub fn upgrade(&mut self) -> Option<Global> {
        self.0.upgrade().map(|x| Global(x))
    }
}

fn find_main_element() -> Option<HtmlElement> {
    for name in vec!{ "body" } {
        let el : Option<Element> = domutil::query_selector_new(name);
        if let Some(el) = el {
            let el : Option<HtmlElement> = el.try_into().ok();
            if let Some(h) = el {
                return Some(h);   
            }
        }
    }
    None
}

pub fn setup_global() {
    let inst_bytes = (browser_time() as i64).to_be_bytes();
    let mut inst_id = base64::encode_config(&inst_bytes,base64::URL_SAFE_NO_PAD);
    let len = inst_id.len();
    let inst_id = inst_id.split_off(len-6);
    set_instance_id(&inst_id);
    let g = Arc::new(Mutex::new(Global::new()));
    register_startup_events(&g);
    register_shutdown_events(&g);
    if let Some(h) = find_main_element() {
        h.focus();
        domutil::add_attr(&h.clone().into(),"class","browser-app-ready");
        domutil::remove_attr(&h.into(),"class","browser-app-not-ready");
    }
}
