use std::sync::{ Arc, Mutex };
use controller::global::{ App, AppRunner };
use controller::input::{ Action,actions_run };
use types::CDFraction;

pub struct OpticalImpl {
    missing: f64,
    pos: Option<(CDFraction,f64)>,
    settled: bool
}

const LETHARGY : f64 = 0.2;
const EPS : f64 = 0.001;

impl OpticalImpl {
    pub fn new() -> OpticalImpl {
        OpticalImpl {
            missing: 0.,
            pos: None,
            settled: false
        }
    }

    fn send_delta(&mut self, app: &mut App, amt: f64) {
        if let Some((pos,prop)) = self.pos {
            actions_run(app,&vec! {
                Action::Zoom(amt),
                Action::Pos(pos,Some(prop))
            });
        }
    }

    fn tick(&mut self, app: &mut App, _t: f64) {
        if self.missing.abs() > EPS {
            let this_time = self.missing * LETHARGY;
            self.missing -= this_time;
            self.send_delta(app,this_time);
            self.settled = false;
        } else if !self.settled {
            app.with_stage(|s| s.settle());
            self.settled = true;
        }
    }
        
    /* when mouse moves, so does the handle */
    fn shift_handle_by(&mut self, at: f64, pos: CDFraction, prop: f64) {
        self.missing += at;
        self.pos = Some((pos,prop));
    }
}

#[derive(Clone)]
pub struct Optical(Arc<Mutex<OpticalImpl>>);

impl Optical {
    pub fn new(ru: &mut AppRunner) -> Optical {
        let out = Optical(Arc::new(Mutex::new(OpticalImpl::new())));
        let c = out.clone();
        ru.add_timer("optical",move |cg,t,_| { c.clone().tick(cg,t); vec!{} },1);
        out
    }

    pub fn tick(&mut self, cg: &mut App, t: f64) {
        self.0.lock().unwrap().tick(cg,t);
    }

    pub fn move_by(&mut self, by: f64, pos: CDFraction, prop: f64) {
        self.0.lock().unwrap().shift_handle_by(by,pos,prop);
    }
}
