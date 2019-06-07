use std::collections::HashMap;

use composit::{ Leaf, Position, Wrapping };
use controller::output::{ Report, ViewportReport };
use program::UniformValue;
use types::{
    CPixel, cpixel, Move, Dot, Direction, 
    LEFT, RIGHT, UP, DOWN, IN, OUT
};

// XXX TODO avoid big-minus-big type calculations which accumulate error

#[derive(Debug)]
pub struct Stage {
    dims: Dot<f64,f64>,
    mouse_pos: CPixel,
    base: f64,
    pos: Position,
}

impl Stage {
    pub fn new() -> Stage {
        let size = Dot(0.,0.);
        Stage {
            pos: Position::new(Dot(0.,0.),size),
            mouse_pos: Dot(0,0),
            base: 0.,
            dims: size,
        }
    }

    fn bumped(&self, direction: &Direction) -> bool {
        let mul : f64 = direction.1.into();
        self.pos.get_edge(direction).floor() * mul >= self.pos.get_limit_of_edge(direction).floor() * mul
    }

    pub fn update_report(&self, report: &Report) {
        let (left,right) = (self.pos.get_edge(&LEFT),self.pos.get_edge(&RIGHT));
        report.set_status("start",&left.floor().to_string());
        report.set_status("end",&right.ceil().to_string());
        report.set_status_bool("bumper-left",self.bumped(&LEFT));
        report.set_status_bool("bumper-right",self.bumped(&RIGHT));
        report.set_status_bool("bumper-top",self.bumped(&UP));
        report.set_status_bool("bumper-bottom",self.bumped(&DOWN));
        report.set_status_bool("bumper-in",self.bumped(&IN));
        report.set_status_bool("bumper-out",self.bumped(&OUT));
    }

    pub fn update_viewport_report(&self, report: &ViewportReport) {
        report.set_delta_y(-self.pos.get_edge(&UP) as i32);
    }

    pub fn set_wrapping(&mut self, w: &Wrapping) {
        self.pos.set_bumper(&LEFT,w.get_bumper(&LEFT));
        self.pos.set_bumper(&RIGHT,w.get_bumper(&RIGHT));
    }

    pub fn set_mouse_pos(&mut self, c: &CPixel) {
        self.mouse_pos = *c;
    }
    
    pub fn set_screen_in_bp(&mut self, zoom: f64) {
        self.pos.set_screen_in_bp(zoom);
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

    pub fn get_size(&self) -> Dot<f64,f64> {
        self.dims
    }

    pub fn set_size(&mut self, size: &Dot<f64,f64>) {
        self.dims = *size;
        self.pos.inform_screen_size(size);
    }

    pub fn get_uniforms(&self, leaf: &Leaf, opacity: f32) -> HashMap<&str,UniformValue> {
        let bp_per_screen = self.pos.get_linear_zoom();
        let bp_per_leaf = leaf.total_bp();
        let leaf_per_screen = bp_per_screen as f64 / bp_per_leaf;
        let middle_bp = self.pos.get_middle();
        let middle_leaf = middle_bp.0/bp_per_leaf; // including fraction of leaf
        let current_leaf_left = (leaf.get_index() as f64);
        hashmap! {
            "uOpacity" => UniformValue::Float(opacity),
            "uStageHpos" => UniformValue::Float((middle_leaf - current_leaf_left) as f32),
            "uStageVpos" => UniformValue::Float(middle_bp.1 as f32),
            "uStageZoom" => UniformValue::Float((2_f64/leaf_per_screen) as f32),
            "uSize" => UniformValue::Vec2F(
                self.dims.0 as f32/2.,
                self.dims.1 as f32/2.)
        }
    }
}
