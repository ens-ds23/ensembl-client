use std::collections::HashSet;
use std::sync::{ Arc, Mutex };

use core::BinaryCode;
use util::ValueStore;
use super::environment::Environment;
use super::process::Process;

struct InterpProcess(Process);

impl InterpProcess {
    fn send_finished(&mut self, env: &mut Box<Environment>) {
        let exit_float = self.0.get_reg_float(1);
        let exit_str = self.0.get_reg_str(2);  
        env.finished(self.0.get_pid().unwrap(),exit_float,exit_str);
    }
    
    fn run_proc(&mut self, env: &mut Box<Environment>) -> bool {
        self.0.step();
        if self.0.halted() {
            self.send_finished(env);
        }
        return self.0.ready()
    }
    
    fn set_pid(&mut self, pid: usize) {
        self.0.set_pid(pid);
    }
}

#[derive(Clone)]
pub struct Signals {
    awoke: Arc<Mutex<Vec<usize>>>
}

impl Signals {
    pub fn new() -> Signals {
        Signals {
            awoke: Arc::new(Mutex::new(Vec::<usize>::new()))
        }
    }
    
    pub fn awoke(&self, pid: usize) {
        self.awoke.lock().unwrap().push(pid);
    }
    
    pub fn drain_awoken(&self) -> Vec<usize> {
        let out = &mut self.awoke.lock().unwrap();
        let out : Vec<usize> = out.drain(..).collect();
        out.clone()
    }
}

pub struct Interp {
    env: Box<Environment>,
    procs: ValueStore<InterpProcess>,
    runq: HashSet<usize>,
    nextq: HashSet<usize>,
    signals: Signals
}

#[derive(PartialEq)]
enum RunResult { Timeout, Empty, Finished }

impl Interp {
    pub fn new(env: Box<Environment>) -> Interp {
        Interp {
            env,
            procs: ValueStore::<InterpProcess>::new(),
            runq: HashSet::<usize>::new(),
            nextq: HashSet::<usize>::new(),
            signals: Signals::new()
        }
    }
    
    pub fn exec(&mut self, bc: &BinaryCode, start: Option<&str>) -> Result<usize,String> {
        match bc.exec(start,Some(self.signals.clone())) {
            Ok(p) => {
                let pid = self.procs.store(InterpProcess(p));
                self.procs.get_mut(pid).set_pid(pid);
                self.runq.insert(pid);
                Ok(pid)
            },
            Err(e) => Err(e)
        }
    }
    
    fn add_awoken(&mut self) {
        for pid in self.signals.drain_awoken() {
            self.runq.insert(pid);
        }
    }
    
    fn drain_runq(&mut self, end: i64) -> RunResult {
        if self.runq.is_empty() { return RunResult::Empty; }
        let runnable : Vec<usize> = self.runq.drain().collect();
        for pid in runnable {
            let mut ip = self.procs.get_mut(pid);
            if ip.run_proc(&mut self.env) {
                self.nextq.insert(pid);
            }
            if self.env.get_time() > end {
                return RunResult::Timeout;
            }
        }
        RunResult::Finished
    }
    
    fn step(&mut self, end: i64) -> bool {
        loop {
            self.add_awoken();
            let r = self.drain_runq(end);
            if r == RunResult::Finished {
                self.runq = self.nextq.clone();
                self.nextq.clear();
                continue;
            }
            return r == RunResult::Timeout;
        }
    }
}

#[cfg(test)]
mod test {
    use std::{ thread, time };
    use super::Interp;
    use super::super::environment::{ DebugEnvironment, Environment };
    use test::command_compile;
    
    #[test]
    fn noprocs() {
        let mut t_env = DebugEnvironment::new();
        let now = t_env.get_time();
        let mut t = Interp::new(t_env.make());
        assert!(!t.step(now+1000));
    }
    
    #[test]
    fn smoke() {
        let mut t_env = DebugEnvironment::new();
        let now = t_env.get_time();
        let mut t = Interp::new(t_env.make());
        let bin = command_compile("interp-smoke");
        t.exec(&bin,None).ok().unwrap();
        while t.step(now+1000) {}
        assert_eq!("Success!",t_env.get_exit_str().unwrap());
        assert_eq!([0.,200.].to_vec(),t_env.get_exit_float().unwrap());
    }
    
    #[test]
    fn sleep_wake() {
        let mut t_env = DebugEnvironment::new();
        let now = t_env.get_time();
        let mut t = Interp::new(t_env.make());
        let bin = command_compile("interp-sleep-wake");
        t.exec(&bin,None).ok().unwrap();
        while t.step(now+1000) {}
        thread::sleep(time::Duration::from_millis(500));
        while t.step(now+1000) {}
        assert_eq!("awoke",t_env.get_exit_str().unwrap());
    }
}
