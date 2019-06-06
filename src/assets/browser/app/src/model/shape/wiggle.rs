use types::{ CLeaf, Colour, cleaf };

use drivers::webgl::{
    Facade, FacadeType, ShapeLongInstanceData, 
    TypeToShape, ShapeInstanceDataType,
};

use model::shape::ShapeSpec;

#[derive(Clone,Debug)]
pub struct StretchWiggle {
    pub points: Vec<CLeaf>,
    pub y: i32,
    pub group: Colour
}

impl StretchWiggle {
    pub fn new(points: Vec<CLeaf>, group: Colour, y: i32) -> StretchWiggle {
        StretchWiggle { points, group, y }
    }
}

pub fn stretch_wiggle(p: Vec<CLeaf>, y: i32, colour: &Colour) -> ShapeSpec {
    ShapeSpec::Wiggle(StretchWiggle::new(p,colour.clone(),y))
}

pub struct StretchWiggleTypeSpec {}

impl StretchWiggleTypeSpec {
    fn new_colspec(&self, rd: &ShapeLongInstanceData) -> Colour {
        if let Facade::Colour(c) = rd.facade {
            Some(c)
        } else { None }.unwrap()
    }
}

impl TypeToShape for StretchWiggleTypeSpec {
    fn new_long_shape(&self, rd: &ShapeLongInstanceData) -> Option<ShapeSpec> {
        let mut points = Vec::<CLeaf>::new();
        let mut y_iter = rd.pos_y.iter().cycle();
        for x in &rd.pos_x {
            points.push(cleaf(*x as f32,*y_iter.next().unwrap() as i32));
        }
        let thickness = 1;
        Some(stretch_wiggle(points,thickness,&self.new_colspec(&rd)))
    }
    
    fn get_facade_type(&self) -> FacadeType { FacadeType::Colour }
    fn needs_scale(&self) -> (bool,bool) { (true,true) }
    fn sid_type(&self) -> ShapeInstanceDataType { ShapeInstanceDataType::Long }
}
