mod stretch;
mod glshape;
mod util;
mod canvasidx;
mod glrect;
mod glpoly;
mod spot;
mod texture;
mod glshapebox;
mod glwiggle;

pub use self::glshape::GLShape;

pub use self::util::{
    TypeToShape,
    ShapeInstanceData,
    ShapeInstanceDataType,
    ShapeShortInstanceData,
    ShapeLongInstanceData,
    Facade, FacadeType,
    rectangle_g, rectangle_t, vertices_rect,
    colourspec_to_group
};

pub use self::canvasidx::CanvasIdx;

pub use self::stretch::{
    stretch_texture,
    StretchTextureSpec
};

pub use self::spot::Spot;
