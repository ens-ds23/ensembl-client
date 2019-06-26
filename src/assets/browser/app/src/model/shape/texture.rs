use std::fmt::Debug;

use types::{
    EPixel, Edge, APixel, AxisSense, Dot, CPixel, 
    Anchors, Anchor, cpixel, cleaf
};

use model::shape::{
    DrawingSpec, ShapeSpec, Facade, FacadeType, ShapeInstanceDataType,
    ShapeShortInstanceData, TypeToShape, GenericShape
};

impl GenericShape for TextureSpec {}

#[derive(Clone,Copy,Debug)]
pub enum TexturePosition<T: Clone+Copy+Debug> {
    Pin(Dot<T,i32>),
    Tape(Dot<T,Edge<i32>>),
    Fix(EPixel),
    FixUnderPage(EPixel),
    FixUnderTape(EPixel),
    Page(EPixel),
    PageUnderAll(EPixel),
}

#[derive(Clone)]
pub struct TextureSpec {
    pub origin: TexturePosition<f32>,
    pub offset: CPixel,
    pub anchor: Anchors,
    pub scale: CPixel,
    pub aspec: DrawingSpec
}

pub struct TextureTypeSpec {
    pub sea_x: Option<AxisSense>,
    pub sea_y: Option<AxisSense>,
    pub ship_x: (Option<AxisSense>,i32),
    pub ship_y: (Option<AxisSense>,i32),
    pub under: i32,
    pub scale_x: f32,
    pub scale_y: f32
}

impl TextureTypeSpec {
    fn anchor_pt(&self) -> Anchors {
        Dot(Anchor(self.ship_x.0),Anchor(self.ship_y.0))
    }

    fn new_fix(&self, td: &ShapeShortInstanceData) -> ShapeSpec {
        let origin = cpixel(td.pos_x as i32,td.pos_y)
                        .x_edge(self.sea_x.unwrap())
                        .y_edge(self.sea_y.unwrap());
        let scale = cpixel(self.scale_x as i32,self.scale_y as i32).anchor(self.anchor_pt());
        let offset = cpixel(td.aux_x as i32-self.ship_x.1,
                            td.aux_y as i32-self.ship_y.1);
        let pt = match self.under {
            1 => TexturePosition::FixUnderPage(origin),
            2 => TexturePosition::FixUnderTape(origin),
            _ => TexturePosition::Fix(origin),
        };
        texture(&td.facade,&pt,&scale,&offset)
    }

    fn new_page(&self, td: &ShapeShortInstanceData) -> ShapeSpec {
        let origin = cpixel(td.pos_x as i32,td.pos_y)
                        .x_edge(self.sea_x.unwrap())
                        .y_edge(AxisSense::Max);
        let scale = cpixel(self.scale_x as i32,self.scale_y as i32).anchor(self.anchor_pt());
        let offset = cpixel(td.aux_x as i32-self.ship_x.1,
                            td.aux_y as i32-self.ship_y.1);
        let pt = match self.under {
            3 => TexturePosition::PageUnderAll(origin),
            _ => TexturePosition::Page(origin)
        };
        texture(&td.facade,&pt,&scale,&offset)
    }
    
    fn new_pin(&self, td: &ShapeShortInstanceData) -> Option<ShapeSpec> {
        let origin = cleaf(td.pos_x,td.pos_y);
        let scale = cpixel(self.scale_x as i32,self.scale_y as i32).anchor(self.anchor_pt());
        let offset = cpixel(td.aux_x as i32-self.ship_x.1,
                            td.aux_y as i32-self.ship_y.1);
        if origin.0 <= 1. {
            Some(texture(&td.facade,&TexturePosition::Pin(origin),&scale,&offset))
        } else {
            None
        }
    }
    
    fn new_tape(&self, td: &ShapeShortInstanceData) -> ShapeSpec {
        let origin = cleaf(td.pos_x,td.pos_y).y_edge(self.sea_y.unwrap());
        let scale = cpixel(self.scale_x as i32,self.scale_y as i32).anchor(self.anchor_pt());
        let offset = cpixel(td.aux_x as i32-self.ship_x.1,
                            td.aux_y as i32-self.ship_y.1);
        texture(&td.facade,&TexturePosition::Tape(origin),&scale,&offset)
    }
}

impl TypeToShape for TextureTypeSpec {
    fn new_short_shape(&self, td: &ShapeShortInstanceData) -> Option<ShapeSpec> {
        match (self.sea_x.is_some(),self.sea_y.is_some()) {
            (false,false) => self.new_pin(td),
            (false,true) => Some(self.new_tape(td)),
            (true,false) => Some(self.new_page(td)),
            (true,true) => Some(self.new_fix(td))
        }
    }
    
    fn sid_type(&self) -> ShapeInstanceDataType { ShapeInstanceDataType::Short }
    fn get_facade_type(&self) -> FacadeType { FacadeType::Drawing }
    fn needs_scale(&self) -> (bool,bool) { (self.sea_x.is_none(),false) }
}

fn texture(f: &Facade, origin: &TexturePosition<f32>, scale: &APixel, offset: &CPixel) -> ShapeSpec {
    if let Facade::Drawing(d) = f {
        Some(ShapeSpec::PinTexture(TextureSpec::new(d.clone(),origin,offset,scale)))
    } else { None }.unwrap()
}
