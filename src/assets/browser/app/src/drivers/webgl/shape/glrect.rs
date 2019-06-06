use program::{ ProgramAttribs, PTGeom, PTMethod, ProgramType };
use types::AxisSense;

use super::GLShape;
use super::util::{
    rectangle_p, rectangle_c, rectangle_g, multi_gl, vertices_rect,
    despot, colour, colourspec_to_group
};
use model::shape::{ ColourSpec, RectPosition, RectSpec };
use drivers::webgl::{ GLProgData, Artwork };

fn program_type(spec: &RectSpec) -> PTGeom {
    match spec.offset {
        RectPosition::Pin(_,_)        => PTGeom::Pin,
        RectPosition::Page(_)         => PTGeom::Page,
        RectPosition::PageUnderAll(_) => PTGeom::PageUnderAll,
        RectPosition::Tape(_,_)       => PTGeom::Tape,
        RectPosition::Fix(_)          => PTGeom::Fix,
        RectPosition::FixUnderPage(_) => PTGeom::FixUnderPage,
        RectPosition::FixUnderTape(_) => PTGeom::FixUnderTape,
        RectPosition::Stretch(_)      => PTGeom::Stretch
    }
}

impl GLShape for RectSpec {
    fn into_objects(&self, geom: &mut ProgramAttribs, _art: Option<Artwork>, e: &mut GLProgData) {
        let group = colourspec_to_group(&self.colspec,geom,e);
        let b = vertices_rect(geom,group);
        match self.offset {
            RectPosition::Pin(origin,offset) => {
                rectangle_p(b,geom,"aVertexPosition",&offset);
                multi_gl(b,geom,"aOrigin",&origin,4);
            },
            RectPosition::Page(offset) | 
            RectPosition::PageUnderAll(offset) => {
                let offset = offset.y_edge(AxisSense::Max,AxisSense::Max);
                rectangle_c(b,geom,"aVertexPosition","aVertexSign",&offset);
            },
            RectPosition::Tape(origin,offset) => {
                let offset = offset.x_edge(AxisSense::Max,AxisSense::Max);
                rectangle_c(b,geom,"aVertexPosition","aVertexSign",&offset);
                multi_gl(b,geom,"aOrigin",&origin,4);
            },
            RectPosition::Fix(offset) |
            RectPosition::FixUnderPage(offset) |
            RectPosition::FixUnderTape(offset) => {
                rectangle_c(b,geom,"aVertexPosition","aVertexSign",&offset);
            },
            RectPosition::Stretch(offset) => {
                rectangle_g(b,geom,"aVertexPosition",&offset);
            },
        };
        if let ColourSpec::Colour(c) = self.colspec {
            for _ in 0..4 {
                colour(b,geom,"aVertexColour",&c);
            }
        }
    }
    
    fn get_geometry(&self) -> ProgramType {
        despot(program_type(self),PTMethod::Triangle,&self.colspec)
    }    
}
