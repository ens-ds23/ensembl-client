use std::rc::Rc;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

use drivers::webgl::{ StretchTextureSpec, Artist };
use types::{ Colour, Rect, Placement };
use super::{
    PinPolySpec, StretchWiggle, RectSpec, BoxSpec, BitmapArtist,
    CollageArtist, TextArtist, TextureSpec, ZMenuRectSpec
};

pub trait GenericShape {
    fn zmenu_box(&self) -> Option<(String,Placement)> { None }
}

#[derive(Clone)]
pub enum ShapeSpec {
    PinPoly(PinPolySpec),
    PinRect(RectSpec),
    ZMenu(ZMenuRectSpec),
    PinBox(BoxSpec),
    PinTexture(TextureSpec),
    StretchTexture(StretchTextureSpec),
    Wiggle(StretchWiggle),
}

/* TODO: Why is StretchTextureSpec still in the webgl driver? */
impl GenericShape for ShapeSpec {
    fn zmenu_box(&self) -> Option<(String,Placement)> {
        if let ShapeSpec::ZMenu(s) = self {
            s.zmenu_box()
        } else {
            None
        }
    }
}

#[derive(Clone,Debug)]
pub enum ColourSpec {
    Colour(Colour),
    Spot(Colour),
    ZMenu(String)
}


#[derive(Clone,Copy,Debug)]
pub enum MathsShape {
    Polygon(u16,f32), // (points,offset/rev)
    Circle
}

pub enum ShapeUnder {
    None,
    Page,
    Tape,
    All
}

#[derive(Clone)]
pub enum DrawingSpec {
    Text(TextArtist),
    Bitmap(BitmapArtist),
    Collage(CollageArtist)
}

impl DrawingSpec {
    pub fn to_artist(&self) -> Rc<Artist> {
        match self {
            DrawingSpec::Text(t) => Rc::new(t.clone()),
            DrawingSpec::Bitmap(b) => Rc::new(b.clone()),
            DrawingSpec::Collage(c) => Rc::new(c.clone())
        }
    }
}

pub enum FacadeType {
    Drawing,
    Colour,
    ZMenu
}

#[derive(Clone)]
pub enum Facade {
    Drawing(DrawingSpec),
    Colour(Colour),
    ZMenu(String)
}

pub struct ShapeShortInstanceData {
    pub pos_x: f32,
    pub pos_y: i32,
    pub aux_x: f32,
    pub aux_y: i32,
    pub facade: Facade
}

pub struct ShapeLongInstanceData {
    pub pos_x: Vec<f64>,
    pub pos_y: Vec<f64>,
    pub aux_x: Vec<f64>,
    pub aux_y: Vec<f64>,
    pub facade: Facade
}

pub enum ShapeInstanceDataType {
    Long,
    Short
}

pub enum ShapeInstanceData {
    Short(ShapeShortInstanceData),
    Long(ShapeLongInstanceData)
}

pub trait TypeToShape {
    fn new_short_shape(&self, sid: &ShapeShortInstanceData) -> Option<ShapeSpec> { None }
    fn new_long_shape(&self, sid: &ShapeLongInstanceData) -> Option<ShapeSpec> { None }
    fn get_facade_type(&self) -> FacadeType;
    fn needs_scale(&self) -> (bool,bool);
    fn sid_type(&self) -> ShapeInstanceDataType;
}

#[derive(Clone)]
pub struct DrawingHash(u64);

impl DrawingHash {
    pub fn new<K>(val: K) -> DrawingHash where K : Hash + Eq {
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        DrawingHash(hasher.finish())
    }
    
    pub fn get(&self) -> u64 { self.0 }
}
