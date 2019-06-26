use std::sync::{ Arc, Mutex };

use assembly::{ Argument, Signature };
use core::{ Command, Instruction };
use runtime::{ DataState, ProcState };

#[derive(Debug)]
pub struct Push(usize);

impl Command for Push {    
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let v = {
            let regs = rt.registers();
            regs.get(self.0).clone()
        };
        rt.push_data(v);
        1
    }
}

pub struct PushI();

impl Instruction for PushI {
    fn signature(&self) -> Signature { Signature::new("push","r") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Push(args[0].reg()))
    }
}

#[derive(Debug)]
pub struct Pop(usize);

impl Command for Pop {    
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let v = rt.pop_data();
        let regs = rt.registers();
        regs.set(self.0,v);
        1
    }
}

pub struct PopI();

impl Instruction for PopI {
    fn signature(&self) -> Signature { Signature::new("pop","r") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Pop(args[0].reg()))
    }
}

#[cfg(test)]
mod test {
    use test::{ command_run, TestContext };

    #[test]
    fn pushpop_cmd() {
        let tc = TestContext::new();
        let mut r = command_run(&tc,"pushpop-cmd");
        assert_eq!("[\"hello\"]",r.get_reg(2));
        assert_eq!("[200.0]",r.get_reg(1));
    }    
}
