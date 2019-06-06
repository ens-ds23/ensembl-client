use std::rc::Rc;

use program::{ ProgramAttribs, ProgramType };
use drivers::webgl::GLProgData;
use drivers::webgl::{ Artist, Artwork };
use model::shape::ShapeSpec;

pub trait GLShape {
    fn get_artist(&self) -> Option<Rc<Artist>> { None }
    fn into_objects(&self, geom: &mut ProgramAttribs, art: Option<Artwork>,e: &mut GLProgData);
    fn get_geometry(&self) -> ProgramType;
}

fn as_gl_shape(spec: &ShapeSpec) -> &GLShape {
    match spec {
        ShapeSpec::PinPoly(pp) => pp,
        ShapeSpec::PinRect(pr) => pr,
        ShapeSpec::PinTexture(pt) => pt,
        ShapeSpec::StretchTexture(st) => st,
        ShapeSpec::Wiggle(w) => w,
        ShapeSpec::PinBox(pb) => pb,
    }
}

impl GLShape for ShapeSpec {
    fn get_artist(&self) -> Option<Rc<Artist>> {
        as_gl_shape(self).get_artist()
    }
        
    fn into_objects(&self, geom: &mut ProgramAttribs, art: Option<Artwork>,e: &mut GLProgData) {
        as_gl_shape(self).into_objects(geom,art,e);
    }
    
    fn get_geometry(&self) -> ProgramType {
        as_gl_shape(self).get_geometry()
    }
}
