use std::sync::{ Arc, Mutex };

use assembly::{ Argument, Signature };
use core::{ Command, Instruction, Value };
use runtime::{ DataState, ProcState };

#[derive(Debug)]
pub struct Concat(usize,usize,usize);

impl Command for Concat {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        let mut c : String = regs.get(self.1).as_string(|a| a[0].clone());
        regs.get(self.2).as_string(|b| c.push_str(&b[0]));
        let len = c.len() as i64;
        regs.set(self.0,Value::new_from_string(vec![c]));
        println!("concat cost {}",len);
        len
    }
}

pub struct ConcatI();

impl Instruction for ConcatI {
    fn signature(&self) -> Signature { Signature::new("concat","rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Concat(args[0].reg(),args[1].reg(),args[2].reg()))
    }
}

#[cfg(test)]
mod test {
    use test::{ command_run, TestContext };

    #[test]
    fn commands() {
        let tc = TestContext::new();
        let mut r = command_run(&tc,"concat");
        assert_eq!("[\"hello world!\"]",r.get_reg(3));
    }
}
