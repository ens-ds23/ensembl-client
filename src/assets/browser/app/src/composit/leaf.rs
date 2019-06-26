use std::fmt;

use composit::{ Stick, Scale };
use types::Placement;

#[derive(Clone,PartialEq,Eq,Hash)]
pub struct Leaf {
    stick: Stick,
    hindex: i64,
    scale: Scale
}

impl Leaf {
    pub fn new(stick: &Stick, hindex: i64, scale: &Scale) -> Leaf {
        Leaf { hindex, scale: scale.clone(), stick: stick.clone() }
    }
    
    pub fn containing(stick: &Stick, pos: f64, scale: &Scale) -> Leaf {
        let hindex = (pos / scale.total_bp()).floor() as i64;
        Leaf::new(stick,hindex,scale)
    }
    
    pub fn get_stick(&self) -> &Stick { &self.stick }
    pub fn get_index(&self) -> i64 { self.hindex }
    pub fn get_scale(&self) -> &Scale { &self.scale }    
    
    pub fn total_bp(&self) -> f64 { self.scale.total_bp() }
    
    pub fn get_start(&self) -> f64 {
        (self.get_index() as f64 * self.total_bp()).floor()
    }
    
    pub fn get_end(&self) -> f64 {
        ((self.get_index()+1) as f64 * self.total_bp()).ceil()
    }
    
    pub fn prop(&self, pos: f64) -> f32 {
        let mul = self.total_bp();
        let start_leaf = (self.hindex as f64 * mul) as f64;
        ((pos-start_leaf)/mul) as f32
    }
    
    pub fn unprop(&self, px: f32) -> f64 {
        let mul = self.total_bp();
        let start_leaf = (self.hindex as f64 * mul) as f64;
        px as f64*mul + start_leaf
    }
    
    pub fn get_spec(&self) -> String {
        format!("{}:{}-{}",self.get_stick().get_name(),self.get_start(),self.get_end())
    }
    
    pub fn get_short_spec(&self) -> (String,String) {
        (self.get_stick().get_name(),
         format!("{}{}",self.scale.letter(),self.hindex))
    }
    
    pub fn fix_placement(&self, p: &Placement) -> Placement {
        p.add_bp(self.get_start(),self.total_bp())
    }
}

impl fmt::Debug for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mul = self.total_bp();
        let start = (self.get_index() as f64 * mul).floor() as i32;
        let end = ((self.get_index()+1) as f64 * mul).ceil() as i32;
        write!(f,"{:?}:{}:{:?}[{}-{}]",self.stick,self.hindex,self.scale,start,end)
    }
}
