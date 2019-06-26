use std::rc::Rc;
use std::sync::{ Arc, Mutex };

use core::{ Command, Value };
use super::datastate::DataState;
use super::procconf::ProcessConfig;
use super::procstate::ProcState;
use super::interp::Signals;

pub struct Process {
    program: Rc<Vec<Box<Command>>>,
    data: DataState,
    proc: Arc<Mutex<ProcState>>,
    killed: Option<String>
}

impl Process {
    pub fn new(program: Rc<Vec<Box<Command>>>, pc: usize, 
               signals: Option<Signals>, config: &ProcessConfig) -> Process {
        Process {
            program,
            data: DataState::new(pc,config),
            proc: Arc::new(Mutex::new(ProcState::new(signals))),
            killed: None,
        }
    }
    
    pub fn start(&mut self, pid: usize) {
        self.proc.lock().unwrap().set_pid(pid);
    }
    
    pub fn get_pid(&self) -> Option<usize> { self.proc.lock().unwrap().get_pid() }
    
    pub fn step(&mut self) -> i64 {
        let mut kill = None;
        let cyc = {
            let data = &mut self.data;
            {
                if data.pc() >= self.program.len() {
                    self.proc.lock().unwrap().halt();
                }
                if self.proc.lock().unwrap().is_halted() {
                    return 0;
                }
            }
            data.clear_cont();
            let cyc = self.program[data.pc()].execute(data,self.proc.clone());
            if let Some(msg) = data.is_bust() {
                kill = Some(format!("Exceeded memory limit: {}",msg));
                self.proc.lock().unwrap().halt();
            }
            data.jump(1);
            cyc
        };
        if let Some(kill) = kill {
            self.kill(kill);
        }
        cyc
    }
    
    pub fn kill(&mut self, msg: String) {
        let mut proc = self.proc.lock().unwrap();
        self.killed = Some(msg);
        proc.halt();
    }
    
    pub fn halted(&self) -> bool {
        let proc = self.proc.lock().unwrap();
        proc.is_halted()
    }
    
    pub fn killed(&self) -> Option<String> {
        self.killed.as_ref().map(|s| s.clone())
    }
    
    pub fn ready(&self) -> bool {
        let proc = self.proc.lock().unwrap();
        !proc.is_sleeping() && !proc.is_halted()
    }
    
    pub fn set_remaining(&mut self, remain: i64) {
        self.proc.lock().unwrap().set_remaining(Some(remain));
    }
    
    pub fn run(&mut self) {
        loop {
            if !self.ready() { return; }
            self.step();
        }
    }
    
    pub fn get_reg(&mut self, idx: usize) -> String {
        let val = self.data.registers().get(idx);
        format!("{:?}",val)
    }

    pub fn set_reg(&mut self, idx: usize, v: Value) {
        self.data.registers().set(idx,v);
    }

    pub fn get_reg_str(&mut self, idx: usize) -> Vec<String> {
        self.data.registers().get(idx).as_string(|s| s.clone())
    }

    pub fn get_reg_float(&mut self, idx: usize) -> Vec<f64> {
        self.data.registers().get(idx).as_floats(|f| f.clone())
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use core::Value;
    use runtime::PROCESS_CONFIG_DEFAULT;
    use super::Process;
    
    #[test]
    fn registers() {
        let mut r = Process::new(Rc::new(vec!{}),0,None,&PROCESS_CONFIG_DEFAULT);
        let regs = r.data.registers();
        regs.set(4,Value::new_from_string(vec!["hi".to_string()]));
        let v = regs.get(4);
        regs.set(2,v);
        regs.set(4,Value::new_from_string(vec!["lo".to_string()]));
        assert_eq!("[\"lo\"]",format!("{:?}",regs.get(4)));
        assert_eq!("[\"hi\"]",format!("{:?}",regs.get(2)));
        assert_eq!("[]",format!("{:?}",regs.get(1)));
        regs.drop(4);
        assert_eq!("[]",format!("{:?}",regs.get(4)));
    }

    #[test]
    fn data_stack() {
        let mut r = Process::new(Rc::new(vec!{}),0,None,&PROCESS_CONFIG_DEFAULT);
        r.data.push_data(Value::new_from_string(vec!["lo".to_string()]));
        r.data.push_data(Value::new_from_string(vec!["hi".to_string()]));
        assert_eq!("[\"hi\"]",format!("{:?}",r.data.peek_data()));
        assert_eq!("[\"hi\"]",format!("{:?}",r.data.pop_data()));
        assert_eq!("[\"lo\"]",format!("{:?}",r.data.pop_data()));
        assert_eq!("[]",format!("{:?}",r.data.pop_data()));
    }
}
