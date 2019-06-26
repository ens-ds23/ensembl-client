mod coord;
mod colour;
mod area;
mod corners;

pub use types::corners::{
    Anchors, Axis, AxisSense, Anchored, Corner, Edge, Anchor,
    cedge, Direction,

    LEFT, RIGHT, UP, DOWN, IN, OUT,
    TOPLEFT, BOTTOMLEFT, TOPRIGHT, BOTTOMRIGHT,
    
    A_TOPLEFT,    A_TOP,    A_TOPRIGHT,
    A_LEFT,       A_MIDDLE, A_RIGHT,
    A_BOTTOMLEFT, A_BOTTOM, A_BOTTOMRIGHT
};

pub use types::coord::{
    CPixel, CLeaf, CFraction, APixel, CDFraction,
    cpixel, cleaf, cfraction, cdfraction, ddiv,
    EPixel, Dot, Move, Distance, Units,
};

pub use types::area::{
    RLeaf,
    RFraction,
    RPixel,
    area, Rect, area_size, area_centred,
    Bounds, Placement, XPosition, YPosition
};

pub use types::colour::{
    Colour
};

