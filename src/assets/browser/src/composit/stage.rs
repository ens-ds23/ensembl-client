use std::collections::HashMap;

use composit::{ Leaf, Position };
use program::UniformValue;
use types::{CPixel, cpixel, Move, Dot, Direction };

// XXX TODO avoid big-minus-big type calculations which accumulate error

#[derive(Debug)]
pub struct Stage {
    dims: CPixel,
    mouse_pos: CPixel,
    base: f64,
    pos: Position,
}

impl Stage {
    pub fn new() -> Stage {
        let size = cpixel(0,0);
        let mut out = Stage {
            pos: Position::new(Dot(0.,0.),size),
            mouse_pos: Dot(0,0),
            base: 0.,
            dims: size,
        };
        out
    }

    pub fn set_mouse_pos(&mut self, c: &CPixel) {
        self.mouse_pos = *c;
    }
    
    pub fn settle(&mut self) { self.pos.settle(); }
    
    pub fn get_mouse_pos_prop(&self) -> f64 {
        self.mouse_pos.0 as f64 / self.get_size().0 as f64
    }

    pub fn get_pos_prop_bp(&self, prop: f64) -> f64 {
        let start = self.get_pos_middle().0 - self.pos.get_linear_zoom() / 2.;
        start + prop * self.pos.get_linear_zoom()
    }

    pub fn get_mouse_pos_bp(&self) -> f64 {
        self.get_pos_prop_bp(self.get_mouse_pos_prop())
    }
        
    pub fn pos_prop_bp_to_origin(&self, pos: f64, prop: f64) -> f64 {
        let start = pos - prop * self.pos.get_linear_zoom();
        start + self.pos.get_linear_zoom()/2.
    }

    pub fn set_limit(&mut self, which: &Direction, val: f64) {
        self.pos.set_limit(which,val);
    }
    
    pub fn get_screen_in_bp(&self) -> f64 {
        self.pos.get_screen_in_bp()
    }
    
    pub fn get_pos_middle(&self) -> Dot<f64,f64> {
        self.pos.get_middle()
    }
    
    pub fn inc_pos(&mut self, delta: &Move<f64,f64>) {
        let p = self.pos.get_middle() + *delta;
        self.pos.set_middle(&p);
    }

    pub fn set_zoom(&mut self, v: f64) {
        self.pos.set_zoom(v);
    }

    pub fn inc_zoom(&mut self, by: f64) {
        let z = self.pos.get_zoom() + by;
        self.pos.set_zoom(z);
    }
    
    pub fn get_zoom(&self) -> f64 {
        self.pos.get_zoom()
    }

    pub fn get_linear_zoom(&self) -> f64 {
        self.pos.get_linear_zoom()
    }

    pub fn set_pos_middle(&mut self, pos: &Dot<f64,f64>) {
        self.pos.set_middle(pos);
    }

    pub fn get_size(&self) -> CPixel {
        self.dims
    }

    pub fn set_size(&mut self, size: &CPixel) {
        self.dims = *size;
        self.pos.inform_screen_size(size);
    }

    pub fn get_uniforms(&self, leaf: &Leaf, opacity: f32) -> HashMap<&str,UniformValue> {
        let z = self.pos.get_linear_zoom();
        let zl = leaf.total_bp();
        let ls = z as f64 / zl;
        let middle = self.pos.get_middle();
        let x = middle.0*ls/z as f64;
        hashmap! {
            "uOpacity" => UniformValue::Float(opacity),
            "uStageHpos" => UniformValue::Float((x - leaf.get_index() as f64) as f32),
            "uStageVpos" => UniformValue::Float(middle.1 as f32),
            "uStageZoom" => UniformValue::Float((2_f64/ls) as f32),
            "uSize" => UniformValue::Vec2F(
                self.dims.0 as f32/2.,
                self.dims.1 as f32/2.)
        }
    }
}
