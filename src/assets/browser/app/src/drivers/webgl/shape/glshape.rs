use std::rc::Rc;

use super::super::program::{ ProgramAttribs, ProgramType };
use drivers::webgl::GLProgData;
use drivers::webgl::{ Artist, Artwork };
use model::shape::ShapeSpec;

pub trait GLShape {
    fn get_artist(&self) -> Option<Rc<Artist>> { None }
    fn into_objects(&self, geom: &mut ProgramAttribs, art: Option<Artwork>,e: &mut GLProgData);
    fn get_geometry(&self) -> Option<ProgramType>;
}

fn as_gl_shape(spec: &ShapeSpec) -> Option<&GLShape> {
    match spec {
        ShapeSpec::PinPoly(pp) => Some(pp),
        ShapeSpec::PinRect(pr) => Some(pr),
        ShapeSpec::PinTexture(pt) => Some(pt),
        ShapeSpec::StretchTexture(st) => Some(st),
        ShapeSpec::Wiggle(w) => Some(w),
        ShapeSpec::PinBox(pb) => Some(pb),
        ShapeSpec::ZMenu(zm) => Some(zm)
    }
}

impl GLShape for ShapeSpec {
    fn get_artist(&self) -> Option<Rc<Artist>> {
        as_gl_shape(self).unwrap().get_artist()
    }
        
    fn into_objects(&self, geom: &mut ProgramAttribs, art: Option<Artwork>,e: &mut GLProgData) {
        as_gl_shape(self).unwrap().into_objects(geom,art,e);
    }
    
    fn get_geometry(&self) -> Option<ProgramType> {
        as_gl_shape(self).unwrap().get_geometry()
    }
}
