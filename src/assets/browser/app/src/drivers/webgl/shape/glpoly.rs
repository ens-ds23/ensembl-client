use std::f32;

use super::super::program::{
    ProgramAttribs, DataBatch, PTGeom, PTMethod, ProgramType,
    Input
};

use types::{
    CFraction, cfraction, AxisSense, Bounds, Anchors, RFraction, TOPLEFT
};

use super::GLShape;
use super::util::{
    multi_gl, poly_p, vertices_poly, vertices_hollowpoly, despot,
    colourspec_to_group
};
use drivers::webgl::{ GLProgData, Artwork };
use model::shape::{ ColourSpec, MathsShape, PolyPosition, PinPolySpec };

const CIRC_TOL : f32 = 1.; // max px undercut

fn circle_points(r: f32) -> u16 {
    // 2*sqrt(pi*r) via 2nd order cos approx to max pixel undercut
    (3.54 * (r/CIRC_TOL).sqrt()) as u16
}

fn program_type(spec: &PinPolySpec) -> PTGeom {
    match spec.origin {
        PolyPosition::Fix(_) => PTGeom::Fix,
        PolyPosition::Page(_) => PTGeom::Page,
        PolyPosition::Pin(_) => PTGeom::Pin,
        PolyPosition::Tape(_) => PTGeom::Tape
    }
}

impl GLShape for PinPolySpec {    
    fn into_objects(&self, geom_a: &mut ProgramAttribs, _art: Option<Artwork>, e: &mut GLProgData) {
        /* Convert circles to polygons */
        let (points, offset) = match self.ms {
            MathsShape::Circle => (circle_points(self.size),0.),
            MathsShape::Polygon(p,o) => (p,o)
        };
        /* hollow or solid? */
        let (mt,width,hollow) = match self.width {
            Some(width) => (PTMethod::Strip,width,true),
            None        => (PTMethod::Triangle,0.,false)
        };
        let geom = despot(program_type(&self),mt,&self.colspec);
        let ppd = PinPolyDraw {
            origin: self.origin, 
            size: self.size, 
            colspec: self.colspec.clone(),
            geom: geom,
            anchor: self.anchor,
            points, offset, width, hollow            
        };
        ppd.draw(geom_a, _art, e);
    }

    fn get_geometry(&self) -> Option<ProgramType> {
        let mt = if self.width.is_some() {
            PTMethod::Strip
        } else {
            PTMethod::Triangle
        };
        Some(despot(program_type(&self),mt,&self.colspec))
    }
}

/*
 * PinPoly
 */

struct PinPolyDraw {
    origin: PolyPosition<f32>,
    anchor: Anchors,
    size: f32,
    width: f32,
    points: u16,
    offset: f32,
    colspec: ColourSpec,
    hollow: bool,
    geom: ProgramType
}

impl PinPolyDraw {
    fn add(&self, b: DataBatch, geom: &mut ProgramAttribs, 
           v: Vec<CFraction>, nump: u16) {
        let bbox = self.bounding_box(&v);
        /* Which corner will, ultimately, our axes be based against,
         * ie in which direction will they run?
         */
        let corner = match self.origin {
            PolyPosition::Pin(_) => TOPLEFT,
            PolyPosition::Tape(origin) => origin.x_edge(AxisSense::Max).corner(),
            PolyPosition::Page(origin) => origin.y_edge(AxisSense::Max).corner(),
            PolyPosition::Fix(origin) => origin.corner(),
        };
        /* What's the offset to the MIDDLE of our shape from our
         * chosen anchor point?
         */
        let middle = self.anchor.flip(&corner).to_middle(bbox);
        /* For geometries without an explicit origin, add that in. */
        let delta = match self.origin {
            PolyPosition::Fix(origin) =>
                origin.quantity().as_fraction(),
            PolyPosition::Page(origin) =>
                origin.y_edge(AxisSense::Max).quantity().as_fraction(),
            _ => cfraction(0.,0.)
        };
        /* Transform each of our points according to the middle above,
         * then flipping them, if necessary, to take into account our
         * chosen co-ordinate system (to preserve orientation)
         */
        let bbox = bbox + middle;        
        let v : Vec<CFraction> = v.iter().map(|s|
            bbox.flip_area(*s+middle,corner)+delta
        ).collect();
        let w : Vec<&Input> = v.iter().map(|s| s as &Input).collect();
        match self.origin {
            PolyPosition::Pin(origin) => {
                poly_p(b,geom,"aVertexPosition",&w);
                multi_gl(b,geom,"aOrigin",&origin,nump);
            },
            PolyPosition::Tape(origin) => {
                let origin = origin.x_edge(AxisSense::Max);
                poly_p(b,geom,"aVertexPosition",&w);
                multi_gl(b,geom,"aOrigin",&origin.quantity(),nump);
                multi_gl(b,geom,"aVertexSign",&origin.corner(),nump);
            },
            PolyPosition::Fix(origin) => {
                poly_p(b,geom,"aVertexPosition",&w);
                multi_gl(b,geom,"aVertexSign",&origin.corner(),nump);
            },
            PolyPosition::Page(origin) => {
                poly_p(b,geom,"aVertexPosition",&w);
                multi_gl(b,geom,"aVertexSign",&origin.y_edge(AxisSense::Max).corner(),nump);
            },
        }
        if let ColourSpec::Colour(c) = self.colspec {        
            multi_gl(b,geom,"aVertexColour",&c,nump);
        }
    }
    
    fn build_points(&self, hollow: bool) -> Vec<CFraction> {
        let mut v = Vec::<CFraction>::new();
        let delta = f32::consts::PI * 2. / self.points as f32;
        let mut t = self.offset * f32::consts::PI * 2.;
        if !hollow { v.push(cfraction(0.,0.)); }
        let outer = self.size + self.width;
        for _i in 0..self.points {
            let (x,y) = (t.cos(),t.sin());
            t += delta;
            if hollow {
                v.push(cfraction(x*outer,y*outer));
            }
            v.push(cfraction(x * self.size,y * self.size));
        }
        v
    }

    fn bounding_box(&self, pts: &Vec<CFraction>) -> RFraction {
        let mut b = Bounds::new();
        for p in pts { b.add(*p); }
        b.get().unwrap()
    }    

    fn draw(&self, geom: &mut ProgramAttribs, _art: Option<Artwork>, e: &mut GLProgData) {
        let group = colourspec_to_group(&self.colspec,geom,e);
        if self.hollow {
            let b = vertices_hollowpoly(geom,self.points,group);
            let v = self.build_points(true);
            self.add(b,geom,v,self.points*2);
        } else {
            let b = vertices_poly(geom,self.points,group);
            let v = self.build_points(false);
            self.add(b,geom,v,self.points+1);
        }
    }
}
