use std::collections::HashMap;
use std::rc::Rc;
use stdweb::web::html_element::CanvasElement;
use dom::webgl::{
    WebGLRenderingContext as glctx,
    WebGLTexture as gltex,
    GLint, GLenum,
};

use drivers::webgl::GLProgData;
use super::super::data::DataBatch;
use super::super::objects::Object;

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub enum CanvasWeave {
    Pixelate,
    Blur
}

impl CanvasWeave {
    fn apply(&self, ctx: &glctx) {
        let (minf,magf,wraps,wrapt) = match self {
            CanvasWeave::Pixelate =>
                (glctx::NEAREST,glctx::NEAREST,
                 glctx::CLAMP_TO_EDGE,glctx::CLAMP_TO_EDGE),
            CanvasWeave::Blur =>
                (glctx::LINEAR,glctx::LINEAR,
                 glctx::REPEAT,glctx::REPEAT)
        };
        ctx.tex_parameteri(glctx::TEXTURE_2D,
                           glctx::TEXTURE_MIN_FILTER,
                           minf as i32);
        ctx.tex_parameteri(glctx::TEXTURE_2D,
                           glctx::TEXTURE_MAG_FILTER,
                           magf as i32);
        ctx.tex_parameteri(glctx::TEXTURE_2D,
                           glctx::TEXTURE_WRAP_S,
                           wraps as i32);
        ctx.tex_parameteri(glctx::TEXTURE_2D,
                           glctx::TEXTURE_WRAP_T,
                           wrapt as i32);
    }
}

fn fix_tex_image2_d_cnv(ctx: &glctx, 
                    target: GLenum, level: GLint, internalformat: GLint,
                    format: GLenum, type_: GLenum, canvas: &CanvasElement) {
    js! {
        @{ctx.as_ref()}.texImage2D(@{target}, @{level}, @{internalformat}, 
                           @{format}, @{type_}, @{canvas});
    };
}

fn canvas_texture(ctx: &glctx, cnv : &CanvasElement, w: &CanvasWeave) -> Option<gltex> {
    ctx.create_texture().and_then(|texture| {
        ctx.bind_texture(glctx::TEXTURE_2D, Some(&texture));
        fix_tex_image2_d_cnv(&ctx,
            glctx::TEXTURE_2D,0,glctx::RGBA as i32,
            glctx::RGBA,glctx::UNSIGNED_BYTE,cnv);
        w.apply(ctx);
        Some(texture)
    })
}

/* ObjectCanvasTexture = Object for canvas-origin textures */
pub struct ObjectCanvasTexture {
    textures: HashMap<u32,Rc<gltex>>,
}

impl ObjectCanvasTexture {
    pub fn new() -> ObjectCanvasTexture {
        ObjectCanvasTexture {
            textures: HashMap::<u32,Rc<gltex>>::new()
        }
    }
}

impl Object for ObjectCanvasTexture {
    fn obj_final(&mut self, _batch: &DataBatch, ctx: &glctx, e: &mut GLProgData) {
        let canvs = e.get_canvases().all_ocm();
        let cc = e.get_canvas_cache();
        for c in canvs {
            if cc.find_texture(c).is_none() {
                if let Some(canvas) = c.canvas.as_ref() {
                    if let Some(t) = canvas_texture(ctx,&canvas.element(),&canvas.weave()) {
                        cc.set_texture(c,&t);
                    }
                }
            }
            if let Some(texture) = cc.find_texture(c) {
                self.textures.insert(c.index(),Rc::new(texture));
            }
        }
    }

    fn execute(&mut self, ctx: &glctx, _batch: &DataBatch) {
        for (i,t) in self.textures.iter() {
            ctx.active_texture(glctx::TEXTURE0+*i);
            ctx.bind_texture(glctx::TEXTURE_2D,Some(&t));
        }
    }    
}
