use super::schedrun::SchedRun;
use super::schedtask::SchedTask;
use super::scheduler::Scheduler;

pub struct SchedulerGroup {
    scheduler: Scheduler,
    managed: Vec<u32>
}

impl SchedulerGroup {
    pub(in super) fn new(scheduler: Scheduler) -> SchedulerGroup {
        SchedulerGroup {
            scheduler,
            managed: Vec::new()
        }
    }
    
    pub fn add(&mut self, name: &str, cb: Box<FnMut(&mut SchedRun) + 'static>, prio: usize, on_beat: bool) {
        let task = SchedTask::new(name,cb);
        let id = self.scheduler.add(task,prio,on_beat);
        self.managed.push(id);
    }
}

impl Drop for SchedulerGroup {
    fn drop(&mut self) {
        for id in &self.managed {
            self.scheduler.delete(*id);
        }
    }
}
