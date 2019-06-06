pub use super::schedrun::SchedRun;

pub struct SchedTask {
    stream: String,
    cb: Box<FnMut(&mut SchedRun) + 'static>,
    id: u32
}

impl SchedTask {
    pub(in super) fn new(name: &str, cb: Box<FnMut(&mut SchedRun) + 'static>) -> SchedTask {
        SchedTask { stream: format!("scheduler-task-{}",name), cb, id: 0 }
    }
    
    pub(in super) fn run(&mut self, available: f64) {
        bb_time_if!(&self.stream,{
            let mut sr = SchedRun::new(available);
            (self.cb)(&mut sr);
            sr.was_productive()
        });
    }
    
    pub(in super) fn set_id(&mut self, id: u32) {
        self.id = id;
    }
    
    pub(in super) fn get_id(&self) -> u32 {
        self.id
    }
}
