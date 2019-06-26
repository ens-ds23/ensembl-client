mod bitmap;
mod collage;
mod poly;
mod rect;
mod pinrect;
mod stretchrect;
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
    RectSpec,  ZMenuRectSpec,
    RectPosition, ZPosition, PatinaSpec
};

pub use self::pinrect::{
    PinRectTypeSpec,
};

pub use self::stretchrect::{
    StretchRectTypeSpec, BoxSpec,
};

pub use self::shapespec::{ 
    ColourSpec, DrawingSpec, ShapeSpec, MathsShape, DrawingHash,
    Facade, FacadeType, ShapeInstanceDataType,
    ShapeShortInstanceData, TypeToShape, ShapeLongInstanceData,
    GenericShape
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
