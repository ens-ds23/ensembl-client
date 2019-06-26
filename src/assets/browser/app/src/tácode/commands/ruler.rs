use std::sync::{ Arc, Mutex };

use tánaiste::{
    Argument, Command, DataState, Instruction, ProcState, Signature,
    Value
};

use composit::Leaf;
use tácode::core::{ TáContext, TáTask };
use util::RulerGenerator;

fn extract_config(config: &Vec<f64>) -> (i32,i32,i32,[i32;4]) {
    (config[0] as i32,config[1] as i32,config[2] as i32,
     [config[3] as i32,config[4] as i32,
      config[5] as i32,config[6] as i32])
}

fn build_output(leaf: &Leaf, values: &Vec<(f32,i32,Option<String>)>) -> (Vec<f64>, Vec<f64>, Vec<f64>, String) {
    let mut offsets = Vec::<f64>::new();
    let mut heights = Vec::<f64>::new();
    let mut text_lens = Vec::<f64>::new();
    let mut texts = String::new();
    let empty = "".to_string();
    for (offset,height,text) in values {
        offsets.push(leaf.unprop(*offset));
        heights.push(*height as f64);
        let text = text.as_ref().unwrap_or(&empty);
        text_lens.push(text.len() as f64);
        texts.push_str(&text.clone());
    }
    (offsets,heights,text_lens,texts)
}

fn ruler(leaf: &Leaf, config: &Vec<f64>) -> (Vec<f64>, Vec<f64>, Vec<f64>, String) {
    let (mark_tg, tick_tg, num_tg, heights) = extract_config(config);
    let rg = RulerGenerator::new_leaf(leaf);
    build_output(leaf,&rg.ruler(mark_tg,tick_tg,num_tg,&heights))
}

//  ruler #offset, #height, #text-len, #text, #config
pub struct Ruler(TáContext,usize,usize,usize,usize,usize);

impl Command for Ruler {
    #[allow(irrefutable_let_patterns)]
    fn execute(&self, rt: &mut DataState, proc: Arc<Mutex<ProcState>>) -> i64 {
        let pid = proc.lock().unwrap().get_pid().unwrap();
        self.0.with_task(pid,|task| {
            if let TáTask::MakeShapes(_,leaf,_,_,_,_,_) = task {
                let regs = rt.registers();
                regs.get(self.5).as_floats(|config| {
                     let (offset,height,text_len,text) = ruler(leaf,config);
                     regs.set(self.1,Value::new_from_float(offset));
                     regs.set(self.2,Value::new_from_float(height));
                     regs.set(self.3,Value::new_from_float(text_len));
                     regs.set(self.4,Value::new_from_string(vec![text]));
                });
            }
        });
        return 1;
    }
}

pub struct RulerI(pub TáContext);

impl Instruction for RulerI {
    fn signature(&self) -> Signature { Signature::new("ruler","rrrrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Ruler(self.0.clone(),args[0].reg(),args[1].reg(),
                        args[2].reg(),args[3].reg(),args[4].reg()))
    }
}
