use std::rc::Rc;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

use drivers::webgl::{ StretchTextureSpec, Artist };
use types::Colour;
use super::{
    PinPolySpec, StretchWiggle, RectSpec, BoxSpec, BitmapArtist,
    CollageArtist, TextArtist, TextureSpec
};

#[derive(Clone)]
pub enum ShapeSpec {
    PinPoly(PinPolySpec),
    PinRect(RectSpec),
    PinBox(BoxSpec),
    PinTexture(TextureSpec),
    StretchTexture(StretchTextureSpec),
    Wiggle(StretchWiggle),
}

#[derive(Clone,Copy,Debug)]
pub enum ColourSpec {
    Colour(Colour),
    Spot(Colour),
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
