use drivers::webgl::{ Artwork, GLProgData };
use super::super::shape::GLShape;
use program::{ PTGeom, PTMethod, ProgramType, ProgramAttribs, Input };
use super::util::{
    despot, vertices_hollowpoly, poly_p, multi_gl, colourspec_to_group 
};
use types::{ CFraction, CLeaf, cfraction, cleaf };
use model::shape::{ BoxSpec, ColourSpec };

const DELTA_X: &[f32] = &[0.,1.,0.,1.,0.,-1.,0.,-1.];
const DELTA_Y: &[i32] = &[0, 1, 0,-1, 0, -1, 0,  1];

const END_X: &[f32] = &[0.,0.,0.,0.,1.,1.,1.,1.];
const END_Y: &[i32] = &[0, 0, 1, 1, 1, 1, 0, 0];

fn origins(spec: &BoxSpec) -> Vec<CLeaf> {
    let mut v = Vec::<CLeaf>::new();
    let near = spec.offset.offset();
    let far = spec.offset.far_offset();
    for i in 0..8 {
        v.push(cleaf(near.0*(1.-END_X[i])+far.0*END_X[i],
                     near.1*(1 -END_Y[i])+far.1*END_Y[i]));
    }
    v
}

fn offsets(spec: &BoxSpec) -> Vec<CFraction> {
    let mut v = Vec::<CFraction>::new();
    for i in 0..8 {            
        v.push(cfraction(spec.width as f32*DELTA_X[i],(spec.width*DELTA_Y[i]) as f32));
    }
    v
}

impl GLShape for BoxSpec {
    fn into_objects(&self, geom: &mut ProgramAttribs, _art: Option<Artwork>, e: &mut GLProgData) {
        let group = colourspec_to_group(&self.colspec,geom,e);
        let b = vertices_hollowpoly(geom,4,group);
        let origins = origins(&self);
        let origins : Vec<&Input> = origins.iter().map(|s| s as &Input).collect();
        poly_p(b,geom,"aOrigin",&origins);
        let offsets = offsets(&self);
        let offsets : Vec<&Input> = offsets.iter().map(|s| s as &Input).collect();
        poly_p(b,geom,"aVertexPosition",&offsets);
        if let ColourSpec::Colour(c) = self.colspec {        
            multi_gl(b,geom,"aVertexColour",&c,8);
        }
    }
    
    fn get_geometry(&self) -> ProgramType {
        despot(PTGeom::Pin,PTMethod::Strip,&self.colspec)
    }
}
