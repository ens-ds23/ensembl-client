use dom::domutil::browser_time;

use super::schedtask::SchedTask;

pub(in super) struct SchedQueue {
    tasks: Vec<SchedTask>,
    next: usize
}

impl SchedQueue {
    pub(in super) fn new() -> SchedQueue {
        SchedQueue {
            tasks: Vec::new(),
            next: 0
        }
    }
    
    pub(in super) fn add(&mut self, task: SchedTask) {
        self.tasks.push(task);
    }
    
    pub(in super) fn delete(&mut self, id: u32) {
        loop {
            let mut target = None;        
            for (index,task) in self.tasks.iter().enumerate() {
                if task.get_id() == id {
                    target = Some(index);
                    break;
                }
            }
            if let Some(index) = target {
                self.tasks.remove(index);
            } else {
                break;
            }
        }
    }
    
    pub(in super) fn run(&mut self, end_at: f64) -> bool {
        if self.tasks.len() == 0 { return true; }
        let mut t = browser_time();
        if self.next >= self.tasks.len() { self.next = 0; }
        for idx in 0..self.tasks.len() {
            if t >= end_at { return false; }
            let task = &mut self.tasks[self.next];
            task.run(end_at-t);
            self.next += 1;
            self.next %= self.tasks.len();
            t = browser_time();
        }
        return t < end_at;
    }
}
