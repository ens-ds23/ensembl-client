use dom::domutil::browser_time;

use super::jank::JankBuster;
use super::schedqueuelist::{ SchedNewTask, SchedQueueList };

pub(in super) struct SchedulerMain {
    on_beat: SchedQueueList,
    all_beats: SchedQueueList,
    timesig: u32,
    count: u32,
    jank: JankBuster
}

impl SchedulerMain {
    pub(in super) fn new() -> SchedulerMain {
        SchedulerMain {
            on_beat: SchedQueueList::new("on"),
            all_beats: SchedQueueList::new("all"),
            timesig: 1,
            count: 0,
            jank: JankBuster::new()
        }
    }
    
    pub(in super) fn set_timesig(&mut self, sig: u32) {
        self.timesig = sig;
    }
    
    fn add_tasks(&mut self, mut new: Vec<SchedNewTask>) {
        for task in new.drain(..) {
            if task.on_beat {
                self.on_beat.add(task);
            } else {
                self.all_beats.add(task);
            }
        }
    }
    
    fn del_tasks(&mut self, mut dels: Vec<u32>) {
        for id in dels.drain(..) {
            self.on_beat.delete(id);
            self.all_beats.delete(id);
        }
    }

    fn run_beat(&mut self, allotment: f64) -> bool {
        let end_at = browser_time() + allotment;
        let on_beat = (self.count == 0);
        self.count += 1;
        if self.count >= self.timesig { self.count = 0; }
        bb_metronome!("scheduler-beat");
        let mut busrt = false;
        if on_beat {
            let mut burst = !self.on_beat.run(end_at);
            if !burst {
                let tail_burst = !self.all_beats.run(end_at);
                if self.timesig == 1 {
                    burst |= tail_burst;
                }
            }
            burst
        } else {
            !self.all_beats.run(end_at)
        }
    }
    
    fn check_tempo(&mut self, burst: bool) {
        let now = browser_time();
        self.jank.detect(burst,now/1000.);
        self.set_timesig(self.jank.gear());
    }
    
    pub(in super) fn beat(&mut self, new: Vec<SchedNewTask>, dels: Vec<u32>, allotment: f64) {
        self.add_tasks(new);
        self.del_tasks(dels);
        let busted = self.run_beat(allotment);
        self.check_tempo(busted);
    }
}
