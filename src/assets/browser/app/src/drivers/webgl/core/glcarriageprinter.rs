use std::collections::HashSet;
use std::rc::Rc;

use super::{ GLSourceResponse, GLProgs, GLProgInstances };
use program::ProgramType;
use model::train::Carriage;
use composit::{ Leaf, Stage };
use super::super::drawing::{ CarriageCanvases, AllCanvasAllocator };
use dom::webgl::WebGLRenderingContext as glctx;

pub struct GLCarriagePrinter {
    srr: HashSet<GLSourceResponse>,
    prev_cc: Option<CarriageCanvases>,
    leaf: Leaf,
    progs: Option<GLProgs>,
    ctx: Rc<glctx>
}

impl GLCarriagePrinter {
    pub fn new(acm: &mut AllCanvasAllocator, leaf: &Leaf, progs: &GLProgs, ctx: &Rc<glctx>) -> GLCarriagePrinter {
        GLCarriagePrinter {
            srr: HashSet::<GLSourceResponse>::new(),
            prev_cc: None,
            leaf: leaf.clone(),
            progs: Some(progs.clean_instance()),
            ctx: ctx.clone()
        }
    }

    pub fn new_sr(&mut self, sr: &GLSourceResponse) {
        self.srr.insert(sr.clone());
    }

    pub fn remove_sr(&mut self, sr: &mut GLSourceResponse) {
        self.srr.remove(sr);
    }
    
    pub fn destroy(&mut self, alloc: &mut AllCanvasAllocator) {
        if let Some(cc) = self.prev_cc.take() {
            cc.destroy(alloc);
        }
    }
        
    fn redraw_drawings(&mut self, alloc: &mut AllCanvasAllocator) -> CarriageCanvases {
        let mut cc = alloc.make_carriage_canvases();
        for sr in self.srr.iter() {
            sr.redraw_drawings(&mut cc);
        }
        cc.finalise(alloc);
        cc
    }
    
    fn redraw_objects(&mut self,e: &mut GLProgInstances) {
        for sr in self.srr.iter() {
            sr.redraw_objects(e);
        }
    }

    fn redraw_travellers(&mut self, aca: &mut AllCanvasAllocator) {
        if let Some(prev_cc) = self.prev_cc.take() {
            prev_cc.destroy(aca);
        }
        let cc = self.redraw_drawings(aca);
        let mut e = GLProgInstances::new(cc,self.progs.take().unwrap(),&self.ctx);
        self.redraw_objects(&mut e);
        e.finalize_objects(&self.ctx);
        e.go();
        let (prev_cc,progs) = e.destroy();
        self.prev_cc = Some(prev_cc);
        self.progs = Some(progs);
    }
    
    pub fn prepare(&mut self,
                        carriage: &mut Carriage,
                        aca: &mut AllCanvasAllocator,
                        stage: &Stage, opacity: f32) {
        if carriage.needs_refresh() {
            carriage.reset_needs_refresh();
            self.redraw_travellers(aca);
        }
        let progs = self.progs.as_mut().unwrap();
        for k in &progs.order {
            let prog = progs.map.get_mut(k).unwrap();
            let u = stage.get_uniforms(&self.leaf, opacity);
            for (key, value) in &u {
                if let Some(obj) = prog.get_object(key) {
                    obj.set_uniform(None,*value);
                }
            }
        }
    }
    
    pub fn execute(&mut self, pt: &ProgramType) {
        let prog = self.progs.as_mut().unwrap().map.get_mut(pt).unwrap();
        prog.execute(&self.ctx);
    }    
}
