use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use runtime::{ Environment, ProcessState };

pub struct DebugEnvironmentExtern {
    start: Instant,
    pid: Option<usize>,
    exit_state: Option<ProcessState>,
    last_exit_str: Vec<String>,
    last_exit_float: Vec<Vec<f64>>
}

impl DebugEnvironmentExtern {
    pub fn new() -> DebugEnvironmentExtern {
        DebugEnvironmentExtern {
            start: Instant::now(),
            pid: None,
            exit_state: None,
            last_exit_str: Vec::<String>::new(),
            last_exit_float: Vec::<Vec<f64>>::new()
        }
    }
    
    pub fn get_pid(&self) -> Option<usize> { self.pid }
    pub fn get_exit_str(&self) -> &Vec<String> { &self.last_exit_str }
    pub fn get_exit_float(&self) -> &Vec<Vec<f64>> { &self.last_exit_float }
    pub fn get_exit_state(&self) -> &Option<ProcessState> { &self.exit_state }
}

impl Environment for DebugEnvironmentExtern {
    fn get_time(&mut self) -> i64 {
        let d = Instant::now().duration_since(self.start);
        d.as_secs() as i64 * 1000 + d.subsec_millis() as i64
    }
    
    fn started(&mut self, pid: usize) {
        self.pid = Some(pid);
    }
    
    fn finished(&mut self, _pid: usize, state: ProcessState, codes: Vec<f64>, string: Vec<String>) {
        self.exit_state = Some(state);
        self.last_exit_str.push(string[0].clone());
        self.last_exit_float.push(codes);
    }
}

pub struct DebugEnvironmentBox(Rc<RefCell<DebugEnvironmentExtern>>);

impl Environment for DebugEnvironmentBox {
    fn get_time(&mut self) -> i64 {
        self.0.borrow_mut().get_time()
    }
    
    fn started(&mut self, pid: usize) {
        self.0.borrow_mut().started(pid);
    }
    
    fn finished(&mut self, pid: usize, state: ProcessState, codes: Vec<f64>, string: Vec<String>) {
        self.0.borrow_mut().finished(pid, state, codes, string);
    }
}

pub struct DebugEnvironment(Rc<RefCell<DebugEnvironmentExtern>>);

impl DebugEnvironment {
    pub fn new() -> DebugEnvironment {
        DebugEnvironment(Rc::new(RefCell::new(DebugEnvironmentExtern::new())))
    }

    pub fn make(&self) -> Box<Environment> {
        Box::new(DebugEnvironmentBox(self.0.clone()))
    }

    pub fn get_exit_str(&self) -> Vec<String> {
        self.0.borrow_mut().get_exit_str().clone()
    }
    pub fn get_exit_float(&self) -> Vec<Vec<f64>> {
        self.0.borrow_mut().get_exit_float().clone()
    }
    pub fn get_exit_state(&self) -> Option<ProcessState> {
        self.0.borrow_mut().get_exit_state().clone()
    }
    pub fn get_pid(&self) -> Option<usize> {
        self.0.borrow_mut().get_pid()
    }
}
