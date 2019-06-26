use std::sync::{ Arc, Mutex };

use assembly::{ Argument, Signature };
use core::{ Command, Instruction, Value };
use runtime::{ DataState, ProcState };

#[derive(Debug)]
pub struct Constant(usize,Value);

impl Command for Constant {    
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        rt.registers().set(self.0,self.1.clone());
        self.1.len() as i64
    }
}

pub struct ConstantI();

impl Instruction for ConstantI {
    fn signature(&self) -> Signature { Signature::new("const","rc") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Constant(args[0].reg(),args[1].value()))
    }
}

#[derive(Debug)]
pub struct Move(usize,usize);

impl Move {
    pub fn new(dest: usize, src: usize) -> Box<Command> {
        Box::new(Move(dest,src))
    }
}

impl Command for Move {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        let v = regs.get(self.1).clone();
        regs.set(self.0,v);
        return 1;
    }
}

pub struct MoveI();

impl Instruction for MoveI {
    fn signature(&self) -> Signature { Signature::new("move","rr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Move::new(args[0].reg(),args[1].reg())
    }
}

#[cfg(test)]
mod test {    
    use test::{ command_run, TestContext };

    #[test]
    fn const_cmd() {   
        let tc = TestContext::new();
        let mut r = command_run(&tc,"const");
        assert_eq!("[\"hi\"]",r.get_reg(1));
    }
    
    #[test]
    fn move_cmd() {
        let tc = TestContext::new();
        let mut r = command_run(&tc,"move");
        assert_eq!("[\"hi\"]",r.get_reg(2));
    }
}
