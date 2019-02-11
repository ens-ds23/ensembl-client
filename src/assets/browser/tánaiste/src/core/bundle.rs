use super::instruction::Instruction;

use commands::*;

pub struct InstructionBundle(Vec<Box<Instruction>>);

impl InstructionBundle {
    pub fn new() -> InstructionBundle {
        InstructionBundle(Vec::<Box<Instruction>>::new())
    }
    
    pub fn add(&mut self, c: Box<Instruction>) {
        self.0.push(c);
    }

    pub fn add_all(&mut self, mut c: Vec<Box<Instruction>>) {
        self.0.append(&mut c);
    }
    
    pub fn drain(&mut self) -> impl Iterator<Item=Box<Instruction>> {
        self.0.drain(..)
    }
}

pub fn instruction_bundle_core() -> InstructionBundle {
    let mut ib = InstructionBundle::new();
    ib.add_all(vec! {
        Box::new(ConcatI()) as Box<Instruction>,
        Box::new(ConstantI()),
        Box::new(DPrintI()), 
        Box::new(ExternalI()), 
        Box::new(HaltI()),
        Box::new(MoveI()),
        Box::new(SleepI()),
        Box::new(PushI())
    });
    ib
}
