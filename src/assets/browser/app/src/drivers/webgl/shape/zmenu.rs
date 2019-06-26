use super::{ GLShape };
use super::util::{
    points_g, vertices_strip, colourspec_to_group
};

use super::super::program::{ PTGeom, PTMethod, PTSkin, ProgramType, ProgramAttribs };
use drivers::webgl::{ GLProgData, Artwork };
use model::shape::{ ColourSpec, StretchWiggle, ZMenuRectSpec };

impl GLShape for ZMenuRectSpec {
    fn into_objects(&self, geom: &mut ProgramAttribs, _art: Option<Artwork>, e: &mut GLProgData) {
    }
    
    fn get_geometry(&self) -> Option<ProgramType> { 
        None
    }
}
