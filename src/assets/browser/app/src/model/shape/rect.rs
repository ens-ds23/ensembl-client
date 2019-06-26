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

#[derive(Clone,Debug)]
pub struct RectSpec {
    pub offset: RectPosition,
    pub colspec: ColourSpec
}

#[derive(Clone,Debug)]
pub struct ZMenuRectSpec {
    pub offset: RectPosition,
    pub id: String
}

impl GenericShape for RectSpec {}

impl GenericShape for ZMenuRectSpec {
    fn zmenu_box(&self) -> Option<(String,Placement)> {
        Some((self.id.to_string(),self.offset.0))
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum PatinaSpec {
    Colour,
    Spot,
    ZMenu
}
