use std::rc::Rc;

use super::super::program::{ ProgramType, PTGeom, PTSkin, PTMethod, ProgramAttribs };
use types::{
    APixel, AxisSense, CPixel, area_centred, cfraction
};

use drivers::webgl::GLShape;
use super::util::{ rectangle_t, multi_gl, vertices_rect };

use drivers::webgl::{ GLProgData, Artist, Artwork };
use model::shape::{ DrawingSpec, TexturePosition, TextureSpec };

impl GLShape for TextureSpec {
    fn into_objects(&self, geom: &mut ProgramAttribs, 
                    artwork: Option<Artwork>, e: &mut GLProgData) {
        if let Some(art) = artwork {
            let group = e.canvas().get_group(geom,&art.weave);
            let mut mp = art.mask_pos;
            let mut ap = art.pos;
            let mut anchor = self.anchor;
            let mut pos = cfraction(0.,0.);
            let mut offset = self.offset.as_fraction();
            let mut bounds_check = false;
            match self.origin {
                TexturePosition::Pin(_) => {
                    bounds_check = true;
                },
                TexturePosition::Tape(origin) => {
                    let origin = origin.x_edge(AxisSense::Max);
                    ap = ap.flip_d(origin);
                    mp = mp.flip_d(origin);
                    offset = offset.flip(&origin.corner());
                    anchor = anchor.flip(&origin.corner());
                },
                TexturePosition::Fix(origin) |
                TexturePosition::FixUnderPage(origin) |
                TexturePosition::FixUnderTape(origin) |
                TexturePosition::Page(origin) |
                TexturePosition::PageUnderAll(origin) => {
                    ap = ap.flip_d(origin);
                    mp = mp.flip_d(origin);
                    anchor = anchor.flip(&origin.corner());
                    pos = origin.quantity().as_fraction();
                }
            }
            let p = area_centred(pos,
                                 (art.size * self.scale).as_fraction());
            let p = anchor.to_middle(p) + p + offset;
            if !bounds_check || p.offset().0 <= 1. && p.far_offset().0 >= 0. {
                let b = vertices_rect(geom,Some(group));
                rectangle_t(b,geom,"aTextureCoord",&ap);
                rectangle_t(b,geom,"aMaskCoord",&mp);
                rectangle_t(b,geom,"aVertexPosition",&p);
                match self.origin {
                    TexturePosition::Pin(origin) => {
                        multi_gl(b,geom,"aOrigin",&origin,4);
                    },
                    TexturePosition::Tape(origin) => {
                        let origin = origin.x_edge(AxisSense::Max);
                        multi_gl(b,geom,"aOrigin",&origin.quantity(),4);
                        multi_gl(b,geom,"aVertexSign",&origin.corner(),4);
                    },
                    TexturePosition::Fix(origin) |
                    TexturePosition::FixUnderPage(origin) |
                    TexturePosition::FixUnderTape(origin) |
                    TexturePosition::Page(origin) |
                    TexturePosition::PageUnderAll(origin) => {
                        multi_gl(b,geom,"aVertexSign",&origin.corner(),4);
                    }
                }
            }
        }
    }

    fn get_geometry(&self) -> Option<ProgramType> {
        let gt = match self.origin {
            TexturePosition::Pin(_) => PTGeom::Pin,
            TexturePosition::Tape(_) => PTGeom::Tape,
            TexturePosition::Fix(_) => PTGeom::Fix,
            TexturePosition::FixUnderPage(_) => PTGeom::FixUnderPage,
            TexturePosition::FixUnderTape(_) => PTGeom::FixUnderTape,
            TexturePosition::Page(_) => PTGeom::Page,
            TexturePosition::PageUnderAll(_) => PTGeom::PageUnderAll,
        };
        Some(ProgramType(gt,PTMethod::Triangle,PTSkin::Texture))
    }

    fn get_artist(&self) -> Option<Rc<Artist>> { 
        Some(self.aspec.to_artist())
    }
}

impl TextureSpec {
    pub fn new(aspec: DrawingSpec, origin: &TexturePosition<f32>, 
           offset: &CPixel, scale: &APixel) -> TextureSpec {
        TextureSpec {
            aspec, origin: *origin, offset: *offset, 
            scale: scale.quantity(),
            anchor: scale.corner()
        }
    }        
}
