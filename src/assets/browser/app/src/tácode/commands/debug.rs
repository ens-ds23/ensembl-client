use std::sync::{ Arc, Mutex };

use tánaiste::{
    Argument, Command, DataState, Instruction, ProcState, Signature
};

//                      reg   console=t/debug=f
pub struct ConsolePrint(usize,bool);

impl Command for ConsolePrint {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        if self.1 {
            console!("{:?}",rt.registers().get(self.0));
        } else {
            bb_log!("bytecode","{:?}",rt.registers().get(self.0));
        }
        return 10
    }
}

pub struct CPrintI();
pub struct DPrintI();

impl Instruction for CPrintI {
    fn signature(&self) -> Signature { Signature::new("cprint","r") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(ConsolePrint(args[0].reg(),true))
    }
}

impl Instruction for DPrintI {
    fn signature(&self) -> Signature { Signature::new("dprint","r") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(ConsolePrint(args[0].reg(),false))
    }
}
