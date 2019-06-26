use std::sync::{ Arc, Mutex };

use tánaiste::{
    Argument, Command, DataState, Instruction, ProcState, Signature,
    Value
};

use model::shape::{ text_texture, DrawingSpec };
use drivers::webgl::{ FCFont, FontVariety
     };
use tácode::{ TáContext, TáTask };
use types::{ Colour };

fn process_meta(font_name: &str,meta: &Vec<f64>) -> (FCFont,Colour,Colour) {
    let variety = match meta[1].round() as i64 {
        1 => FontVariety::Bold,
        _ => FontVariety::Normal
    };
    let fgd = Colour(meta[2] as u32,meta[3] as u32,meta[4] as u32);
    let bgd = Colour(meta[5] as u32,meta[6] as u32,meta[7] as u32);
    (FCFont::new(meta[0].round() as i32,font_name,variety),fgd,bgd)
}

fn text(txx: &mut Vec<DrawingSpec>, font: &FCFont, fgd: &Colour, bgd: &Colour, string: &String) -> usize {
    let tx = text_texture(string,&font,fgd,bgd);
    txx.push(tx);
    txx.len()-1
}

fn texts(tx: &mut Vec<DrawingSpec>, font_name: &str, meta: &Vec<f64>,
         strings: &String, lens: &Vec<f64>) -> Vec<f64> {
    let mut out = Vec::<f64>::new();
    let (font,fgd,bgd) = process_meta(font_name,meta);
    let chars : Vec<char> = strings.chars().collect();
    let mut start = 0;
    for len in lens {
        let len = *len as usize;
        let s = chars[start..start+len].iter().collect();
        start += len;
        out.push(text(tx,&font,&fgd,&bgd,&s) as f64);
    }
    out
}

fn texts2(tx: &mut Vec<DrawingSpec>, font_name: &str, meta: &Vec<f64>,
         strings: &Vec<String>) -> Vec<f64> {
    let (font,fgd,bgd) = process_meta(font_name,meta);
    let mut out = Vec::<f64>::new();
    for s in strings {
        out.push(text(tx,&font,&fgd,&bgd,&s) as f64);
    }
    out
}

// text #target, #font, #meta, #texts, #sizes
pub struct Text(TáContext,usize,usize,usize,usize,usize);

impl Command for Text {
    #[allow(irrefutable_let_patterns)]
    fn execute(&self, rt: &mut DataState, proc: Arc<Mutex<ProcState>>) -> i64 {
        let pid = proc.lock().unwrap().get_pid().unwrap();
        self.0.with_task(pid,|task| {
            if let TáTask::MakeShapes(_,_,_,ref mut tx,_,_,_) = task {
                let regs = rt.registers();
                regs.get(self.2).as_string(|font_name| {
                    regs.get(self.3).as_floats(|meta| {
                        regs.get(self.4).as_string(|strings| {
                            regs.get(self.5).as_floats(|lens| {
                                regs.set(self.1,Value::new_from_float(texts(tx,&font_name[0],meta,&strings[0],lens)));
                            });
                        });
                    });
                });
            }
        });
        return 1;
    }
}

pub struct TextI(pub TáContext);

impl Instruction for TextI {
    fn signature(&self) -> Signature { Signature::new("text","rrrrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Text(self.0.clone(),args[0].reg(),args[1].reg(),
                      args[2].reg(),args[3].reg(),args[4].reg()))
    }
}

// text2 #target, #font, #meta, #texts
pub struct Text2(TáContext,usize,usize,usize,usize);

impl Command for Text2 {
    #[allow(irrefutable_let_patterns)]
    fn execute(&self, rt: &mut DataState, proc: Arc<Mutex<ProcState>>) -> i64 {
        let pid = proc.lock().unwrap().get_pid().unwrap();
        self.0.with_task(pid,|task| {
            if let TáTask::MakeShapes(_,_,_,ref mut tx,_,_,_) = task {
                let regs = rt.registers();
                regs.get(self.2).as_string(|font_name| {
                    regs.get(self.3).as_floats(|meta| {
                        regs.get(self.4).as_string(|strings| {
                            regs.set(self.1,Value::new_from_float(texts2(tx,&font_name[0],meta,strings)));
                        });
                    });
                });
            }
        });
        return 1;
    }
}

pub struct Text2I(pub TáContext);

impl Instruction for Text2I {
    fn signature(&self) -> Signature { Signature::new("text2","rrrrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Text2(self.0.clone(),args[0].reg(),args[1].reg(),
                      args[2].reg(),args[3].reg()))
    }
}
