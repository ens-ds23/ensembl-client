mod core;
mod drawing;
pub mod program; /* XXX only while stage has a ref to the driver */
mod shape;

pub use self::core::{ 
    GLDrawing, GLProgs, GLPrinter, GLProgData
};
pub use self::drawing::{
    Drawing, CarriageCanvases, Artist, Artwork,
    FCFont, FontVariety, Mark,
    collage, mark_rectangle, OneCanvasManager
};
pub use self::shape::{
    stretch_texture,
    rectangle_g, rectangle_t, vertices_rect,
    GLShape, StretchTextureSpec,
};
