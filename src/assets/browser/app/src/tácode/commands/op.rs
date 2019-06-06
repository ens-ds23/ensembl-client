use std::sync::{ Arc, Mutex };

use tánaiste::{
    Argument, Command, DataState, Instruction, ProcState, Signature,
    Value
};

#[derive(Clone)]
pub enum BinOpType {
    Add,
    Mul,
    Div,
    Eq,
    Or,
    Max,
    Min
}

impl BinOpType {
    fn op(&self, a: f64, b: f64) -> f64 {
        match self {
            BinOpType::Add => a+b,
            BinOpType::Mul => a*b,
            BinOpType::Div => a/b,
            BinOpType::Eq  => if a==b { 1. } else { 0. },
            BinOpType::Or => if a != 0. || b != 0. { 1. } else { 0. },
            BinOpType::Max => if a > b { a } else { b },
            BinOpType::Min => if a < b { a } else { b }
        }
    }
    fn name(&self) -> &str {
        match self {
            BinOpType::Add => "add",
            BinOpType::Mul => "mul",
            BinOpType::Div => "div",
            BinOpType::Eq  => "eq",
            BinOpType::Or => "or",
            BinOpType::Max => "max",
            BinOpType::Min => "min"
        }
    }
}

fn binop(type_: &BinOpType, a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    if b.len() == 0 { return a.to_vec(); }
    let mut out = Vec::<f64>::new();
    let mut b_iter = b.iter().cycle();
    for av in a.iter() {
        out.push(type_.op(*av,*b_iter.next().unwrap()));
    }
    out
}

fn member(av: &Vec<f64>, bv: &Vec<f64>) -> Vec<f64> {
    let mut cv : Vec<f64> = Vec::<f64>::new();
    let ai = av.iter().map(|x| x.round() as i64);
    let mut bi = bv.iter().map(|x| x.round() as i64);
    let mut b = bi.next();
    for a in ai {
        while b < Some(a) && b.is_some() {
            b = bi.next();
        }
        if b.is_none() { break; }
        if b > Some(a) {
            cv.push(0.);
        } else if b == Some(a) {
            cv.push(1.);
        }
    }
    cv
}

// add Add #a+b, #a, #b
// mul Mul #a*b, #a, #b
pub struct BinOp(BinOpType,usize,usize,usize);
// member #a(in)b, #a, #b
pub struct Member(usize,usize,usize);

impl Command for BinOp {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.2).as_floats(|a| {
            regs.get(self.3).as_floats(|b| {
                let data = binop(&self.0,a,b);
                regs.set(self.1,Value::new_from_float(data));
            });
        });
        return 1;
    }
}

impl Command for Member {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|a| {
            regs.get(self.2).as_floats(|b| {
                regs.set(self.0,Value::new_from_float(member(a,b)));
            });
        });
        return 1;
    }
}

pub struct BinOpI(pub BinOpType);
impl Instruction for BinOpI {
    fn signature(&self) -> Signature { Signature::new(self.0.name(),"rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(BinOp(self.0.clone(),args[0].reg(),args[1].reg(),args[2].reg()))
    }
}

pub struct MemberI();
impl Instruction for MemberI {
    fn signature(&self) -> Signature { Signature::new("member","rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Member(args[0].reg(),args[1].reg(),args[2].reg()))
    }
}
