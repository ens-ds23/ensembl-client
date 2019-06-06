use std::sync::{ Arc, Mutex };

use tánaiste::{
    assemble, BinaryCode, DEFAULT_CONFIG, instruction_bundle_core,
    InstructionSet, Interp, ProcessConfig, PROCESS_CONFIG_DEFAULT,
    Value
};

use super::appenv::AppEnv;
use tácode::{ instruction_bundle_app, TáContext };

pub struct TácodeImpl {
    tc: TáContext,
    interp: Interp,
    insset: InstructionSet,
    procconf: ProcessConfig
}

fn app_interp(tc: &TáContext) -> Interp {
    let mut cfg = DEFAULT_CONFIG.clone();
    cfg.cycles_per_run = 1000000;
    let env = Box::new(AppEnv::new(tc.clone()));
    Interp::new(env,cfg)
}

fn instruction_set(tc: &TáContext) -> InstructionSet {
    InstructionSet::new(vec!{
        instruction_bundle_core(),
        instruction_bundle_app(tc)
    })
}

fn process_config() -> ProcessConfig {
    let pc = PROCESS_CONFIG_DEFAULT.clone();
    pc
}

impl TácodeImpl {
    pub fn new() -> TácodeImpl {
        let tc = TáContext::new();
        TácodeImpl {
            tc: tc.clone(),
            interp: app_interp(&tc),
            insset: instruction_set(&tc),
            procconf: process_config()
        }
    }

    pub fn assemble(&self, code: &str) -> Result<BinaryCode,Vec<String>> {
        assemble(&self.insset,code)
    }

    pub fn run(&mut self, bin: &BinaryCode) -> Result<usize,String> {
        self.interp.exec(bin,None,Some(&self.procconf))
    }
    
    pub fn step(&mut self, remaining: f64) { 
        self.interp.run(remaining.round() as i64);
    }
    
    pub fn start(&mut self, pid: usize) { self.interp.start(pid); }
    
    pub fn context(&self) -> TáContext {
        self.tc.clone()
    }

    pub fn set_reg(&mut self, pid: usize, reg: usize, v: Value) {
        self.interp.set_reg(pid,reg,v);
    }
}

#[derive(Clone)]
pub struct Tácode(Arc<Mutex<TácodeImpl>>);

impl Tácode {
    pub fn new() -> Tácode {
        Tácode(Arc::new(Mutex::new(TácodeImpl::new())))
    }

    pub fn assemble(&self, code: &str) -> Result<BinaryCode,Vec<String>> {
        self.0.lock().unwrap().assemble(code)
    }

    pub fn run(&self, bin: &BinaryCode) -> Result<usize,String> {
        self.0.lock().unwrap().run(bin)
    }

    pub fn context(&self) -> TáContext {
        self.0.lock().unwrap().context()
    }

    pub fn start(&self, pid: usize) {
        self.0.lock().unwrap().start(pid)
    }
    
    pub fn step(&self, remaining: f64) {
        self.0.lock().unwrap().step(remaining)
    }
    
    pub fn set_reg(&self, pid: usize, reg: usize, v: Value) {
        self.0.lock().unwrap().set_reg(pid, reg, v);
    }
}
