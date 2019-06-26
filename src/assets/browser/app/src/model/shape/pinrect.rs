use std::fmt::Debug;

use types::{ 
    CLeaf, AxisSense, Rect, Edge, RLeaf, area_size, cleaf, cpixel,
    Placement, XPosition, YPosition, Dot
};
use model::shape::{ 
    ColourSpec, ShapeSpec, Facade, FacadeType, ShapeInstanceDataType,
    ShapeShortInstanceData, TypeToShape, GenericShape, PatinaSpec,
    ZPosition, RectSpec, RectPosition, ZMenuRectSpec
};

pub struct PinRectTypeSpec {
    pub sea_x: Option<(AxisSense,AxisSense)>,
    pub sea_y: Option<(AxisSense,AxisSense)>,
    pub ship_x: (Option<AxisSense>,i32),
    pub ship_y: (Option<AxisSense>,i32),
    pub under: i32,
    pub spot: PatinaSpec
}

impl PinRectTypeSpec {
    fn new_colspec(&self, rd: &ShapeShortInstanceData) -> Option<ColourSpec> {
        if let Facade::Colour(c) = rd.facade {
            match self.spot {
                PatinaSpec::Spot => Some(ColourSpec::Spot(c)),
                PatinaSpec::Colour => Some(ColourSpec::Colour(c)),
                _ => None
            }
        } else if let Facade::ZMenu(ref c) = rd.facade {
            Some(ColourSpec::ZMenu(c.to_string()))
        } else {
            None
        }
    }
    
    fn new_pin_delta(&self, len: i32, ship: (Option<AxisSense>,i32)) -> i32 {
        match ship.0 {
            Some(AxisSense::Min) => 0,
            Some(AxisSense::Max) => len,
            None => len/2
        }
    }
    
    fn new_pin_offset(&self, rd: &ShapeShortInstanceData) -> Rect<i32,i32> {
        let size = cpixel(rd.aux_x as i32,rd.aux_y);
        let delta_x = self.new_pin_delta(size.0,self.ship_x);
        let delta_y = self.new_pin_delta(size.1,self.ship_y);
        area_size(cpixel(-delta_x,-delta_y),size)
    }
    
    fn new_pin(&self, rd: &ShapeShortInstanceData) -> Option<ShapeSpec> {
        let offset = self.new_pin_offset(rd);
        let colspec = self.new_colspec(rd);
        let nw = offset.offset();
        let se = offset.far_offset();
        Some(ShapeSpec::PinRect(RectSpec {
            offset: RectPosition(Placement::Placed(
                        XPosition::Base(rd.pos_x as f64,nw.0,se.0),
                        YPosition::Page(rd.pos_y+nw.1,rd.pos_y+se.1)),
                        ZPosition::Normal),
            colspec: colspec.unwrap()
        }))
    }
    
    fn new_tape(&self, rd: &ShapeShortInstanceData) -> Option<ShapeSpec> {
        let offset = self.new_pin_offset(rd) + Dot(0,rd.pos_y);
        let offset = offset.y_edge(self.sea_y.unwrap().0,
                                self.sea_y.unwrap().1);
        let nw = offset.offset();
        let se = offset.far_offset();
        let colspec = self.new_colspec(rd);
        Some(ShapeSpec::PinRect(RectSpec {
            offset: RectPosition(Placement::Placed(
                        XPosition::Base(rd.pos_x as f64,nw.0,se.0),
                        YPosition::Pixel(nw.1,se.1)),
                        ZPosition::Normal),
            colspec: colspec.unwrap()
        }))     
    }

    fn new_page(&self, rd: &ShapeShortInstanceData) -> Option<ShapeSpec> {
        let pos =  (cpixel(rd.pos_x as i32,rd.pos_y) +
                    self.new_pin_offset(rd))
                        .x_edge(self.sea_x.unwrap().0,
                                self.sea_x.unwrap().1);
        let colspec = self.new_colspec(rd);
        let z = match self.under {
            3 => ZPosition::UnderAll,
            _ => ZPosition::Normal
        };
        let nw = pos.offset();
        let se = pos.far_offset();
        
        if self.spot == PatinaSpec::ZMenu {
            if let Some(ColourSpec::ZMenu(id)) = colspec {
                Some(ShapeSpec::ZMenu(ZMenuRectSpec {
                    offset: RectPosition(Placement::Placed(
                                XPosition::Pixel(nw.0,se.0),
                                YPosition::Page(nw.1,se.1)),
                                z),
                    id: id.to_string()
                }))     
            } else {
                None
            }
        } else {        
            Some(ShapeSpec::PinRect(RectSpec {
                offset: RectPosition(Placement::Placed(
                            XPosition::Pixel(nw.0,se.0),
                            YPosition::Page(nw.1,se.1)),
                            z),
                colspec: unwrap!(colspec)
            }))     
        }
    }

    fn new_fix(&self, rd: &ShapeShortInstanceData) -> Option<ShapeSpec> {
        let pos =  (cpixel(rd.pos_x as i32,rd.pos_y) +
                    self.new_pin_offset(rd))
                        .x_edge(self.sea_x.unwrap().0,
                                self.sea_x.unwrap().1)
                        .y_edge(self.sea_y.unwrap().0,
                                self.sea_y.unwrap().1);
        let colspec = self.new_colspec(rd);
        let z = match self.under {
            1 => ZPosition::UnderPage,
            2 => ZPosition::UnderTape,
            _ => ZPosition::Normal
        };    
        let nw = pos.offset();
        let se = pos.far_offset();
        Some(ShapeSpec::PinRect(RectSpec {
            offset: RectPosition(Placement::Placed(
                        XPosition::Pixel(nw.0,se.0),
                        YPosition::Pixel(nw.1,se.1)),
                        z),
            colspec: colspec.unwrap()
        }))     
    }    
}

impl TypeToShape for PinRectTypeSpec {
    fn new_short_shape(&self, sid: &ShapeShortInstanceData) -> Option<ShapeSpec> {
        match (self.sea_x.is_some(),self.sea_y.is_some()) {
            (false,false) =>self.new_pin(sid),
            (false,true) => self.new_tape(sid),
            (true,false) => self.new_page(sid),
            (true,true) => self.new_fix(sid),
        }
    }
    
    fn get_facade_type(&self) -> FacadeType { 
        if self.spot == PatinaSpec::ZMenu {
            FacadeType::ZMenu
        } else {
            FacadeType::Colour
        }
    }
    fn needs_scale(&self) -> (bool,bool) { (self.sea_x.is_none(),false) }
    fn sid_type(&self) -> ShapeInstanceDataType { ShapeInstanceDataType::Short }
}
