use std::rc::Rc;

use types::{ RLeaf, cfraction, cleaf, area_size };
use super::GLShape;
use super::util::{ rectangle_g, rectangle_t, vertices_rect };

use super::super::program::{ PTGeom, PTMethod, PTSkin, ProgramType, ProgramAttribs };
use drivers::webgl::GLProgData;
use drivers::webgl::{ Artist, Artwork };

use model::shape::{ DrawingSpec, ShapeSpec };

#[derive(Clone)]
pub struct StretchTextureSpec {
    pos: RLeaf,
    aspec: DrawingSpec
}

impl StretchTextureSpec {
    pub fn new(aspec: DrawingSpec,pos: &RLeaf) -> StretchTextureSpec {
        StretchTextureSpec {
            pos: *pos, aspec
        }
    }
}

impl StretchTextureSpec {
    pub fn create(&self) -> Box<GLShape> {
        Box::new(self.clone())
    }
}

const CHUNK_SIZE : f32 = 10.;

impl GLShape for StretchTextureSpec {
    fn into_objects(&self, geom: &mut ProgramAttribs, artwork: Option<Artwork>, e: &mut GLProgData) {
        if let Some(art) = artwork {
            /* some cards baulk at very large textured areas, so split */
            let mut chunks = ((self.pos.area()).0.abs() / CHUNK_SIZE) as i32;
            if chunks < 1 { chunks = 1; }
            
            let widthp = (self.pos.area()).0 / chunks as f32;
            let widtht = art.pos.area() / cfraction(chunks as f32,1.);
            let widthtm = art.mask_pos.area() / cfraction(chunks as f32,1.);
            
            let mut p = area_size(self.pos.offset(),cleaf(widthp,(self.pos.area()).1));
            let mut t = area_size(art.pos.offset(),widtht);
            let mut tm = area_size(art.mask_pos.offset(),widthtm);
            for _i in 0..chunks {
                let group = e.canvas().get_group(geom,&art.weave);
                let b = vertices_rect(geom,Some(group));
                rectangle_g(b,geom,"aVertexPosition",&p);
                rectangle_t(b,geom,"aTextureCoord",&t);
                rectangle_t(b,geom,"aMaskCoord",&tm);
                p = p + cleaf(widthp,0);
                t = t + cfraction(widtht.0,0.);
                tm = tm + cfraction(widtht.0,0.);
            }
        }
    }
    
    fn get_geometry(&self) -> Option<ProgramType> {
        Some(ProgramType(PTGeom::Stretch,PTMethod::Triangle,PTSkin::Texture))
    }

    fn get_artist(&self) -> Option<Rc<Artist>> { Some(self.aspec.to_artist()) }
}

pub fn stretch_texture(a: &DrawingSpec, pos: &RLeaf) -> ShapeSpec {
    ShapeSpec::StretchTexture(StretchTextureSpec::new(a.clone(),pos))
}
