use std::sync::{ Arc, Mutex };

use tánaiste::{
    Argument, Command, DataState, Instruction, ProcState, Signature,
    Value
};

use composit::{ Leaf };
use model::shape::{
    Facade, FacadeType, TypeToShape, ShapeShortInstanceData,
    ShapeInstanceDataType, ShapeLongInstanceData, DrawingSpec
};
use composit::source::SourceResponse;
use tácode::core::{ TáContext, TáTask };
use super::super::shapecmd::{ build_meta };
use types::Colour;

fn do_scale(spec: &Box<TypeToShape>, leaf: &Leaf, x_start: f64, x_aux: f64) -> Option<(f32,f32)> {
    let needs_scale = spec.needs_scale();
    let x_pos_v = if needs_scale.0 {
        let p = leaf.prop(x_start);
        if p <= 1. { Some(p) } else { None }
    } else {
        Some(x_start as f32)
    };
    
    let x_aux_v = if needs_scale.1 {
        let p =leaf.prop(x_start+x_aux)-leaf.prop(x_start);
        if p >= 0. { Some(p) } else { None }
    } else {
        Some(x_aux as f32)
    };
    if x_pos_v.is_some() && x_aux_v.is_some() {
        Some((x_pos_v.unwrap(),x_aux_v.unwrap()))
    } else {
        None
    }
}

fn make_colour_facade(spec: &Box<TypeToShape>, colour: &Vec<f64>, tx: &Vec<DrawingSpec>, i: usize) -> Facade {
    let col_len = colour.len();
    match spec.get_facade_type() {
        FacadeType::Colour => {
            let r = colour[(i*3)%col_len] as u32;
            let g = colour[(i*3+1)%col_len] as u32;
            let b = colour[(i*3+2)%col_len] as u32;
            Facade::Colour(Colour(r,g,b))
        },
        FacadeType::Drawing => {
            let idx = colour[i%col_len];
            Facade::Drawing(tx[idx as usize].clone())
        },
        FacadeType::ZMenu => panic!("bad call 1")
    }
}

fn make_facade(spec: &Box<TypeToShape>, colour: &Value, tx: &Vec<DrawingSpec>, i: usize) -> Facade {
    match spec.get_facade_type() {
        FacadeType::Colour | FacadeType::Drawing => {
            colour.as_floats(|colour| {
                make_colour_facade(spec,colour,tx,i)
            })
        },
        FacadeType::ZMenu => Facade::ZMenu("FIX ME".to_string())
    }    
}

fn make_colour_facades(spec: &Box<TypeToShape>, colour: &Vec<f64>, tx: &Vec<DrawingSpec>) -> Vec<Facade> {
    let mut out = Vec::<Facade>::new();
    let col_len = colour.len();
    let type_ = spec.get_facade_type();
    let num_items = match type_ {
        FacadeType::Colour => col_len/3,
        FacadeType::Drawing => col_len,
        FacadeType::ZMenu => panic!("bad call 3")
    };
    for i in 0..num_items {
        out.push(match type_ {
            FacadeType::Colour => {
                let r = colour[i*3] as u32;
                let g = colour[i*3+1] as u32;
                let b = colour[i*3+2] as u32;
                Facade::Colour(Colour(r,g,b))
            },
            FacadeType::Drawing => {
                let idx = colour[i];
                Facade::Drawing(tx[idx as usize].clone())
            },
            FacadeType::ZMenu => panic!("bad call 2")
        });
    }
    out
}

fn make_facades(spec: &Box<TypeToShape>, colour: &Value, tx: &Vec<DrawingSpec>) -> Vec<Facade> {
    match spec.get_facade_type() {
        FacadeType::Colour | FacadeType::Drawing => {
            colour.as_floats(|colour| {
                make_colour_facades(spec,colour,tx)
            })
        }
        FacadeType::ZMenu => {
            let out : Vec<Facade> = colour.as_string(|ids| {
                ids.iter().map(|x| Facade::ZMenu(x.to_string())).collect()
            });
            out
        }
    }
}

/* TODO switch long to use make_facades. Can do it, but no time */
fn draw_long_shapes(spec: Box<TypeToShape>, leaf: &mut Leaf, lc: &mut SourceResponse, 
                tx: &Vec<DrawingSpec>,x_start: &Vec<f64>,
                x_aux: &Vec<f64>, y_start: &Vec<f64>, y_aux: &Vec<f64>,
                colour: &Value, part: &Option<String>) {
    if colour.len() == 0 { return; }
    let facade = make_facade(&spec,colour,tx,0);
    let mut x_start_scaled = Vec::<f64>::new();
    let mut x_aux_scaled = Vec::<f64>::new();
    let mut x_aux_iter = x_aux.iter().cycle();
    for x in x_start {
        if let Some((x_start_v,x_pos_v)) = do_scale(&spec,leaf,*x,*x_aux_iter.next().unwrap()) {
            x_start_scaled.push(x_start_v as f64);
            x_aux_scaled.push(x_pos_v as f64);
        }
    }
    let data = ShapeLongInstanceData {
        pos_x: x_start_scaled,
        pos_y: y_start.clone(),
        aux_x: x_aux_scaled,
        aux_y: y_aux.clone(),
        facade
    };
    if let Some(shape) = spec.new_long_shape(&data) {
        lc.update_data(part,|data| data.add_shape(shape));
    }
}

fn draw_short_shapes(spec: Box<TypeToShape>, leaf: &mut Leaf, lc: &mut SourceResponse, 
                tx: &Vec<DrawingSpec>,x_start: &Vec<f64>,
                x_aux: &Vec<f64>, y_start: &Vec<f64>, y_aux: &Vec<f64>,
                colour: &Value, part: &Option<String>) {
    if colour.len() == 0 { return; }
    let facades = make_facades(&spec,colour,tx);
    let mut f_iter = facades.iter().cycle();
    let y_start_len = y_start.len();
    let x_aux_len = x_aux.len();
    let y_aux_len = y_aux.len();
    let mut drops = 0;
    for i in 0..x_start.len() {
        if let Some((x_pos_v,x_aux_v)) = 
                do_scale(&spec,leaf,x_start[i],x_aux[i%x_aux_len]) {
            let facade = f_iter.next();
            let data = ShapeShortInstanceData {
                pos_x: x_pos_v,
                pos_y: y_start[i%y_start_len] as i32,
                aux_x: x_aux_v,
                aux_y: y_aux[i%y_aux_len] as i32,
                facade: unwrap!(facade.cloned())
            };
            if let Some(shape) = spec.new_short_shape(&data) {
                lc.update_data(part,|data| data.add_shape(shape));
            } else {
                drops += 1;
            }
        }
    }
    bb_if_log!("performance",{
        bb_log!("performance","{:?} {} shapes",leaf.get_short_spec(),x_start.len());
        if drops * 10 > x_start.len() {
            bb_log!("performance","Excessive drops! {:?} {}/{}",
                leaf.get_short_spec(),
                drops,x_start.len());
        }
    });
}

fn draw_shapes(meta: &Vec<f64>,leaf: &mut Leaf, lc: &mut SourceResponse, 
                tx: &Vec<DrawingSpec>,x_start: &Vec<f64>,
                x_aux: &Vec<f64>, y_start: &Vec<f64>, y_aux: &Vec<f64>,
                colour: &Value, part: &Option<String>) {
    let mut meta_iter = meta.iter().cycle();
    if let Some(spec) = build_meta(&mut meta_iter) {
        match spec.sid_type() {
            ShapeInstanceDataType::Short => 
                draw_short_shapes(spec,leaf,lc,tx,x_start,x_aux,
                                  y_start,y_aux,colour,part),
            ShapeInstanceDataType::Long => 
                draw_long_shapes(spec,leaf,lc,tx,x_start,x_aux,
                                  y_start,y_aux,colour,part),
        }
    }
}

// strect #meta, #x-start, #x-size, #y-start, #y-size, #colour
pub struct Shape(TáContext,usize,usize,usize,usize,usize,usize);

impl Command for Shape {
    #[allow(irrefutable_let_patterns)]
    fn execute(&self, rt: &mut DataState, proc: Arc<Mutex<ProcState>>) -> i64 {
        let pid = proc.lock().unwrap().get_pid().unwrap();
        self.0.with_task(pid,|task| {
            if let TáTask::MakeShapes(_,leaf,lc,ref tx,_,part,_) = task {
                let regs = rt.registers();
                regs.get(self.1).as_floats(|meta| {                
                    regs.get(self.2).as_floats(|x_start| {
                        regs.get(self.3).as_floats(|x_size| {
                            regs.get(self.4).as_floats(|y_start| {
                                regs.get(self.5).as_floats(|y_size| {
                                    draw_shapes(meta,leaf,lc,tx,x_start,x_size,
                                            y_start,y_size,&regs.get(self.6),part);
                                });
                            });
                        });
                    });
                });
            }
        });
        return 1
    }
}

pub struct ShapeI(pub TáContext);

impl Instruction for ShapeI {
    fn signature(&self) -> Signature { Signature::new("shape","rrrrrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Shape(self.0.clone(), args[0].reg(),
            args[1].reg(), args[2].reg(), args[3].reg(), args[4].reg(),
            args[5].reg()))
    }
}
