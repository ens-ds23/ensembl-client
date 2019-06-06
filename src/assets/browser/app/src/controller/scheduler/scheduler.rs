use std::collections::{ HashSet };
use std::sync::{ Arc, Mutex };

use super::schedmain::SchedulerMain;
use super::schedgroup::SchedulerGroup;
use super::schedqueuelist::SchedNewTask;
use super::schedtask::SchedTask;

const UNBURST_TIME : f64 = 60000.; // ms
const MAX_BURSTS : u32 = 2;

struct SchedulerImpl {
    next_id: u32,
    main: Arc<Mutex<SchedulerMain>>,
    adds: Vec<SchedNewTask>,
    dels: HashSet<u32>
}

impl SchedulerImpl {
    fn new() -> SchedulerImpl {
        SchedulerImpl {
            next_id: 0,
            main: Arc::new(Mutex::new(SchedulerMain::new())),
            adds: Vec::new(),
            dels: HashSet::new()
        }
    }    
}

#[derive(Clone)]
pub struct Scheduler(Arc<Mutex<SchedulerImpl>>);

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler(Arc::new(Mutex::new(SchedulerImpl::new())))
    }
    
    fn main(&self) -> Arc<Mutex<SchedulerMain>> {
        unwrap!(self.0.lock()).main.clone()
    }
        
    pub fn set_timesig(&self, sig: u32) {
        self.main().lock().unwrap().set_timesig(sig);
    }
    
    pub fn make_group(&self) -> SchedulerGroup {
        SchedulerGroup::new(self.clone())
    }
    
    fn next_id(&mut self) -> u32 {
        let id = &mut unwrap!(self.0.lock()).next_id;
        *id += 1;
        *id
    }
    
    pub(in super) fn add(&mut self, mut task: SchedTask, prio: usize, on_beat: bool) -> u32 {
        let id = self.next_id();
        task.set_id(id);
        let mut adds = &mut unwrap!(self.0.lock()).adds;
        adds.push(SchedNewTask{task,prio,on_beat});
        id
    }
    
    pub(in super) fn delete(&mut self, id: u32) {
        let mut dels = &mut unwrap!(self.0.lock()).dels;
        dels.insert(id);
    }
    
    pub fn beat(&self, allotment: f64) {
        let main = self.main();
        let adds = unwrap!(self.0.lock()).adds.drain(..).collect();
        let dels = unwrap!(self.0.lock()).dels.drain().collect();
        main.lock().unwrap().beat(adds,dels,allotment);
    }
}
