mod bitmap;
mod collage;
mod poly;
mod rect;
mod shapespec;
mod text;
mod texture;
mod wiggle;

pub use self::bitmap::{ BitmapArtist, bitmap_texture };
pub use self::collage::{ MarkSpec, CollageArtist, RectMark };

pub use self::poly::{
    pin_mathsshape,
    tape_mathsshape,
    fix_mathsshape,
    page_mathsshape,
    PinPolySpec,
    PolyPosition
};

pub use self::rect::{
    BoxSpec,
    RectPosition,
    RectSpec,
    PinRectTypeSpec,
    StretchRectTypeSpec,
};

pub use self::shapespec::{ 
    ColourSpec, DrawingSpec, ShapeSpec, MathsShape, DrawingHash
};

pub use self::text::{
    TextArtist,
    text_texture
};

pub use self::texture::{
    TexturePosition,
    TextureSpec,
    TextureTypeSpec,
};

pub use self::wiggle::{
    stretch_wiggle,
    StretchWiggle,
    StretchWiggleTypeSpec
};
