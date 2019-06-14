use std::fmt::Debug;

use types::{ 
    CLeaf, AxisSense, Rect, Edge, RLeaf, area_size, cleaf, cpixel,
    Placement, XPosition, YPosition, Dot
};
use model::shape::{ 
    ColourSpec, ShapeSpec, Facade, FacadeType, ShapeInstanceDataType,
    ShapeShortInstanceData, TypeToShape, GenericShape
};

#[derive(Clone,Copy,Debug)]
pub enum ZPosition {
    Normal,
    UnderPage,
    UnderTape,
    UnderAll
}

#[derive(Clone,Copy,Debug)]
pub struct RectPosition(pub Placement,pub ZPosition);

#[derive(Clone,Copy,Debug)]
pub struct RectSpec {
    pub offset: RectPosition,
    pub colspec: ColourSpec
}

impl GenericShape for RectSpec {
    fn zmenu_box(&self) -> Option<Placement> {
        Some(self.offset.0)
    }
}

pub enum PatinaSpec {
    Colour,
    Spot,
    ZMenu
}
