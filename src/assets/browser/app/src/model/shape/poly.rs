use std::fmt::Debug;
use types::{ Dot, Edge, Anchors };
use super::{ MathsShape, ColourSpec, ShapeSpec, GenericShape };

#[derive(Clone,Copy,Debug)]
pub enum PolyPosition<T: Clone+Copy+Debug> {
    Pin(Dot<T,i32>),
    Tape(Dot<T,Edge<i32>>),
    Fix(Dot<Edge<i32>,Edge<i32>>),
    Page(Dot<Edge<i32>,i32>)
}

#[derive(Clone,Debug)]
pub struct PinPolySpec {
    pub origin: PolyPosition<f32>,
    pub anchor: Anchors,
    pub size: f32,
    pub width: Option<f32>,
    pub ms: MathsShape,
    pub colspec: ColourSpec
}

impl GenericShape for PinPolySpec {}

#[allow(unused)]
pub fn pin_mathsshape(origin: &Dot<f32,i32>,
                      anchor: Anchors,
                      size: f32, width: Option<f32>, ms: MathsShape,
                      colspec: &ColourSpec) -> ShapeSpec {
    ShapeSpec::PinPoly(PinPolySpec {
        origin: PolyPosition::Pin(*origin),
        colspec: colspec.clone(),
        anchor, size, width, ms
    })
}

pub fn fix_mathsshape(origin: &Dot<Edge<i32>,Edge<i32>>,
                       anchor: Anchors,
                       size: f32, width: Option<f32>, ms: MathsShape,
                       colspec: &ColourSpec) -> ShapeSpec {
    ShapeSpec::PinPoly(PinPolySpec {
        origin: PolyPosition::Fix(*origin),
        colspec: colspec.clone(),
        anchor, size, width, ms
    })
}

pub fn page_mathsshape(origin: &Dot<Edge<i32>,i32>,
                       anchor: Anchors,
                       size: f32, width: Option<f32>, ms: MathsShape,
                       colspec: &ColourSpec) -> ShapeSpec {
    ShapeSpec::PinPoly(PinPolySpec {
        origin: PolyPosition::Page(*origin),
        colspec: colspec.clone(),
        anchor, size, width, ms
    })
}

pub fn tape_mathsshape(origin: &Dot<f32,Edge<i32>>,
                       anchor: Anchors,
                       size: f32, width: Option<f32>, ms: MathsShape,
                       colspec: &ColourSpec) -> ShapeSpec {
    ShapeSpec::PinPoly(PinPolySpec {
        origin: PolyPosition::Tape(*origin),
        colspec: colspec.clone(),
        anchor, size, width, ms
    })
}
