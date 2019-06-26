use super::{ GLShape };
use super::util::{
    points_g, vertices_strip, colourspec_to_group
};

use super::super::program::{ PTGeom, PTMethod, PTSkin, ProgramType, ProgramAttribs };
use drivers::webgl::{ GLProgData, Artwork };
use model::shape::{ ColourSpec, StretchWiggle };

impl GLShape for StretchWiggle {
    fn into_objects(&self, geom: &mut ProgramAttribs, _art: Option<Artwork>, e: &mut GLProgData) {
        let dg = colourspec_to_group(&ColourSpec::Spot(self.group),geom,e);
        let b = vertices_strip(geom,self.points.len() as u16*2,dg);
        points_g(b,geom,"aVertexPosition",&self.points,self.y);
    }
    
    fn get_geometry(&self) -> Option<ProgramType> { 
        Some(ProgramType(PTGeom::Stretch,PTMethod::Strip,PTSkin::Spot))
    }
}
