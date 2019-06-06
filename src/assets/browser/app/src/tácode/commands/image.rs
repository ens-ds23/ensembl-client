use std::rc::Rc;
use std::sync::{ Arc, Mutex };

use base64;
use tánaiste::{
    Argument, Command, DataState, Instruction, ProcState, Signature,
    Value
};

use data::BackendConfig;
use model::shape::{ DrawingSpec, DrawingHash, bitmap_texture };
use tácode::{ TáContext, TáTask };
use types::cpixel;

fn load_asset(cfg: &Rc<BackendConfig>, asset: &str, index: usize) -> Value {
    if let Some(asset) = cfg.get_asset(asset) {
        return asset.get_stream(index).clone();
    } else {
        return Value::new_from_float(vec!{})
    }
}

fn image(txx: &mut Vec<DrawingSpec>, dims: &Vec<f64>, data: &String, cache: &String) -> Vec<f64> {
    let cache = if cache == "" { None } else { Some(DrawingHash::new(cache)) };
    let data = base64::decode(data).ok().unwrap();
    let tx = bitmap_texture(data,cpixel(dims[0] as i32,dims[1] as i32),false,cache);
    txx.push(tx);
    vec! { (txx.len()-1) as f64 }
}

// asset #target, #name, #index
pub struct Asset(TáContext,usize,usize,usize);
// image #target, #width/height, #data
pub struct Image(TáContext,usize,usize,usize,usize);

impl Command for Asset {
    #[allow(irrefutable_let_patterns)]
    fn execute(&self, rt: &mut DataState, proc: Arc<Mutex<ProcState>>) -> i64 {
        let pid = proc.lock().unwrap().get_pid().unwrap();
        self.0.with_task(pid,|task| {
            if let TáTask::MakeShapes(_,_,_,_,_,_,cfg) = task {
                let regs = rt.registers();
                regs.get(self.2).as_string(|name| {
                    regs.get(self.3).as_floats(|index| {
                        regs.set(self.1,load_asset(cfg,name,index[0] as usize));
                    });
                });
            }
        });
        return 1;
    }
}

impl Command for Image {
    #[allow(irrefutable_let_patterns)]
    fn execute(&self, rt: &mut DataState, proc: Arc<Mutex<ProcState>>) -> i64 {
        let pid = proc.lock().unwrap().get_pid().unwrap();
        self.0.with_task(pid,|task| {
            if let TáTask::MakeShapes(_,_,_,ref mut tx,_,_,_) = task {
                let regs = rt.registers();
                regs.get(self.2).as_floats(|dims| {
                    regs.get(self.3).as_string(|data| {
                        regs.get(self.4).as_string(|cache| {
                            regs.set(self.1,Value::new_from_float(image(tx,dims,data,cache)));
                        });
                    });
                });
            }
        });
        return 1;
    }
}

pub struct AssetI(pub TáContext);
pub struct ImageI(pub TáContext);

impl Instruction for AssetI {
    fn signature(&self) -> Signature { Signature::new("asset","rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Asset(self.0.clone(),args[0].reg(),args[1].reg(),
                      args[2].reg()))
    }
}

impl Instruction for ImageI {
    fn signature(&self) -> Signature { Signature::new("image","rrrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Image(self.0.clone(),args[0].reg(),args[1].reg(),
                      args[2].reg(),args[3].reg()))
    }
}
