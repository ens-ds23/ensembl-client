use std::fmt;

use composit::{ Stick, Scale };

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
}

impl fmt::Debug for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mul = self.total_bp();
        let start = (self.get_index() as f64 * mul).floor() as i32;
        let end = ((self.get_index()+1) as f64 * mul).ceil() as i32;
        write!(f,"{:?}:{}:{:?}[{}-{}]",self.stick,self.hindex,self.scale,start,end)
    }
}