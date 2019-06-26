use super::super::program::{ ProgramAttribs, PTGeom, PTMethod, ProgramType };
use types::{ AxisSense, Placement, XPosition, YPosition, area, Dot };
use model::shape::{ ZPosition, RectPosition };

use super::GLShape;
use super::util::{
    rectangle_p, rectangle_c, rectangle_g, multi_gl, vertices_rect,
    despot, colour, colourspec_to_group
};
use model::shape::{ ColourSpec, RectSpec };
use drivers::webgl::{ GLProgData, Artwork };

fn program_type(spec: &RectSpec) -> PTGeom {
    match spec.offset {
        RectPosition(Placement::Placed(XPosition::Pixel(_,_),YPosition::Pixel(_,_)),z) =>
            match z {
                ZPosition::UnderPage => PTGeom::FixUnderPage,
                ZPosition::UnderTape => PTGeom::FixUnderTape,
                _ => PTGeom::Fix,
            },
        RectPosition(Placement::Placed(XPosition::Pixel(_,_),YPosition::Page(_,_)),z) =>
            match z {
                ZPosition::UnderAll => PTGeom::PageUnderAll,
                _ => PTGeom::Page
            },
        RectPosition(Placement::Placed(XPosition::Base(_,_,_),YPosition::Page(_,_)),z) => PTGeom::Pin,
        RectPosition(Placement::Placed(XPosition::Base(_,_,_),YPosition::Pixel(_,_)),z) => PTGeom::Tape,
        RectPosition(Placement::Stretch(_),_) => PTGeom::Stretch
    }
}

impl GLShape for RectSpec {
    fn into_objects(&self, geom: &mut ProgramAttribs, _art: Option<Artwork>, e: &mut GLProgData) {
        let group = colourspec_to_group(&self.colspec,geom,e);
        let b = vertices_rect(geom,group);
        match self.offset {
            RectPosition(Placement::Placed(XPosition::Base(bp,x0,x1),YPosition::Page(y0,y1)),_) => {
                rectangle_p(b,geom,"aVertexPosition",&area(Dot(x0,x1),Dot(y0,y1)));
                multi_gl(b,geom,"aOrigin",&Dot(bp,0),4);
            },
            RectPosition(Placement::Placed(XPosition::Pixel(x0,x1),YPosition::Page(y0,y1)),_) => {
                let offset = area(Dot(x0,y0),Dot(x1,y1));
                let offset = offset.y_edge(AxisSense::Max,AxisSense::Max);
                rectangle_c(b,geom,"aVertexPosition","aVertexSign",&offset);                
            },
            RectPosition(Placement::Placed(XPosition::Base(bp,x0,x1),YPosition::Pixel(y0,y1)),_) => {
                let offset = area(Dot(x0,y0),Dot(x1,y1));
                let offset = offset.x_edge(AxisSense::Max,AxisSense::Max);
                rectangle_c(b,geom,"aVertexPosition","aVertexSign",&offset);                
                multi_gl(b,geom,"aOrigin",&Dot(bp,0),4);
            },
            RectPosition(Placement::Placed(XPosition::Pixel(x0,x1),YPosition::Pixel(y0,y1)),_) => {
                let offset = area(Dot(x0,y0),Dot(x1,y1));
                rectangle_c(b,geom,"aVertexPosition","aVertexSign",&offset);
            },
            RectPosition(Placement::Stretch(offset),_) => {
                rectangle_g(b,geom,"aVertexPosition",&offset);
            },
        };
        if let ColourSpec::Colour(c) = self.colspec {
            for _ in 0..4 {
                colour(b,geom,"aVertexColour",&c);
            }
        }
    }
    
    fn get_geometry(&self) -> Option<ProgramType> {
        Some(despot(program_type(self),PTMethod::Triangle,&self.colspec))
    }    
}
