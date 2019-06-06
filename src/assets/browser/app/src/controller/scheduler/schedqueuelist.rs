use super::schedqueue::SchedQueue;
use super::schedtask::SchedTask;

pub(in super) struct SchedNewTask {
    pub task: SchedTask,
    pub prio: usize,
    pub on_beat: bool
}

pub(in super) struct SchedQueueList {
    queues: Vec<SchedQueue>,
    name: String,
    bb_stream: String
}

impl SchedQueueList {
    pub(in super) fn new(name: &str) -> SchedQueueList {
        SchedQueueList {
            queues: Vec::new(),
            name: name.to_string(),
            bb_stream: format!("scheduler-queuelist-{}",name)
        }
    }
    
    pub(in super) fn add(&mut self, new: SchedNewTask) {
        while self.queues.len() <= new.prio {
            self.queues.push(SchedQueue::new());
        }
        self.queues[new.prio].add(new.task);
    }
    
    pub(in super) fn delete(&mut self, id: u32) {
        for q in &mut self.queues {
            q.delete(id);
        }
    }
    
    pub(in super) fn run(&mut self, end_at: f64) -> bool {
        bb_time!(&self.bb_stream,{
            for q in &mut self.queues {
                if !q.run(end_at) {
                    return false;
                }
            }
            return true;
        })
    }
}
