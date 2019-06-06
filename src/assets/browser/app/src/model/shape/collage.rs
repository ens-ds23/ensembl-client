use types::{ CPixel, RPixel, Colour };
use drivers::webgl::Mark;

#[derive(Clone,Debug)]
pub struct RectMark {
    pub coords: RPixel,
    pub colour: Colour
}

#[derive(Clone,Debug)]
pub enum MarkSpec {
    Rect(RectMark)
}

impl MarkSpec {
    pub fn to_mark(&self) -> Box<Mark> {
        match self {
            MarkSpec::Rect(rm) => Box::new(rm.clone())
        }
    }
}

#[derive(Clone,Debug)]
pub struct CollageArtist {
    pub parts: Vec<MarkSpec>,
    pub size: CPixel
}

impl CollageArtist {
    pub fn new(parts: Vec<MarkSpec>, size: CPixel) -> CollageArtist {
        CollageArtist { parts, size }
    }
}

