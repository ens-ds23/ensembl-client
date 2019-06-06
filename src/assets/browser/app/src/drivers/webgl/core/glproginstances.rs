use super::{ GLProgs, GLProgData };
use super::super::drawing::CarriageCanvases;

use dom::webgl::WebGLRenderingContext as glctx;

pub struct GLProgInstances {
    e: GLProgData,
    progs: GLProgs
}

impl GLProgInstances {
    pub fn new(cc: CarriageCanvases, mut progs: GLProgs, ctx: &glctx) -> GLProgInstances {
        progs.clear_objects(ctx);
        GLProgInstances {
            e: GLProgData::new(cc),
            progs
        }
    }
    
    pub fn get_progs_data(&mut self) -> (&mut GLProgs, &mut GLProgData) {
        (&mut self.progs,&mut self.e)
    }
    
    pub fn get_progs(&mut self) -> &mut GLProgs {
        &mut self.progs
    }
    
    pub fn get_data(&mut self) -> &mut GLProgData {
        &mut self.e
    }
    
    pub fn destroy(self) -> (CarriageCanvases,GLProgs) {
        (self.e.destroy(),self.progs)
    }
    
    pub fn go(&mut self) {
        let (progs,data) = self.get_progs_data();
        data.go(progs);
    }
    
    pub fn finalize_objects(&mut self, ctx: &glctx) {
        self.progs.finalize_objects(ctx,&mut self.e);
    }
}
