use std::collections::{ HashMap, HashSet };
use std::rc::Rc;

use stdweb::unstable::TryInto;
use stdweb::web::{ HtmlElement, Element, INode, IElement };

use print::{ Programs, LeafPrinter };
use composit::{ Compositor, Train, StateManager, Leaf, Stage };
use drawing::{ AllCanvasAllocator };
use dom::domutil;
use types::{ Dot };

use dom::webgl::WebGLRenderingContext as glctx;
use stdweb::web::html_element::{
    CanvasElement
};

pub struct Printer {
    canv_el: HtmlElement,
    ctx: Rc<glctx>,
    base_progs: Programs,
    acm: AllCanvasAllocator,
    lp: HashMap<Leaf,LeafPrinter>
}

impl Printer {
    pub fn new(canv_el: &HtmlElement) -> Printer {
        let canvas = canv_el.clone().try_into().unwrap();
        let ctx: glctx = domutil::get_context(&canvas);
        ctx.clear_color(1.0,1.0,1.0,1.0);
        ctx.clear(glctx::COLOR_BUFFER_BIT  | glctx::DEPTH_BUFFER_BIT);
        let ctx_rc = Rc::new(ctx);
        let progs = Programs::new(&ctx_rc);
        let acm = AllCanvasAllocator::new(".bpane-container .managedcanvasholder");
        Printer {
            canv_el: canv_el.clone(),
            acm, ctx: ctx_rc,
            base_progs: progs,
            lp: HashMap::<Leaf,LeafPrinter>::new()
        }
    }

    pub fn finish(&mut self) {
        for (_i,mut lp) in &mut self.lp {
            lp.finish(&mut self.acm);
        }
        self.acm.finish();
    }

    fn create_new_leafs(&mut self, leafs: &Vec<Leaf>) {
        for leaf in leafs.iter() {
            if !self.lp.contains_key(leaf) {
                let progs = self.base_progs.clean_instance();
                self.lp.insert(leaf.clone(),LeafPrinter::new(&mut self.acm,leaf,&progs,&self.ctx));
            }
        }
    }

    fn remove_old_leafs(&mut self, leafs: &Vec<Leaf>) {
        let mut ls = HashSet::new();
        for leaf in leafs {
            ls.insert(leaf);
        }
        let keys : Vec<Leaf> = self.lp.keys().map(|s| s.clone()).collect();
        for leaf in keys {
            if !ls.contains(&leaf) {
                if let Some(mut lp) = self.lp.remove(&leaf) {
                    lp.finish(&mut self.acm);
                }
            }
        }
    }

    fn manage_leafs(&mut self, c: &mut Compositor) {
        let leafs = c.all_printing_leafs();
        self.create_new_leafs(&leafs);
        self.remove_old_leafs(&leafs);        
    }

    fn prepare_all(&mut self) {
        self.ctx.enable(glctx::DEPTH_TEST);
        self.ctx.enable(glctx::BLEND);
        self.ctx.blend_func_separate(
            glctx::SRC_ALPHA,
            glctx::ONE_MINUS_SRC_ALPHA,
            glctx::ONE,
            glctx::ONE_MINUS_SRC_ALPHA);        
        self.ctx.depth_mask(false);
        self.ctx.clear(glctx::COLOR_BUFFER_BIT | glctx::DEPTH_BUFFER_BIT);
    }

    fn prepare_scale(&mut self, stage: &Stage, oom: &StateManager, 
                     sc: &mut Train, opacity: f32) {
        let leafs = sc.leafs();
        for ref leaf in &leafs {
            if let Some(lp) = &mut self.lp.get_mut(&leaf) {
                let redo = sc.calc_level(leaf,oom);
                lp.into_objects(&leaf,sc,&mut self.acm,redo);
                lp.take_snap(stage,opacity);
            }
        }
    }
        
    fn execute(&mut self, c: &mut Compositor) {
        let leafs = c.all_printing_leafs();
        for pt in &self.base_progs.order {
            for ref leaf in &leafs {
                let lp = &mut self.lp.get_mut(&leaf).unwrap();
                lp.execute(&pt);
            }
        }
    }

    pub fn go(&mut self, stage: &Stage, oom: &StateManager, compo: &mut Compositor) {
        self.manage_leafs(compo);
        self.prepare_all();
        let prop = compo.get_prop_trans();
        if let Some(current_train) = compo.get_current_train() {
            self.prepare_scale(stage,oom,current_train,1.-prop);
        }
        if let Some(transition_train) = compo.get_transition_train() {
            self.prepare_scale(stage,oom,transition_train,prop);
        }
        self.execute(compo);
    }
        
    pub fn set_size(&mut self, s: Dot<i32,i32>) {
        let elel: Element =  self.canv_el.clone().into();
        let elc : CanvasElement = elel.clone().try_into().unwrap();
        elc.set_width(s.0 as u32);
        elc.set_height(s.1 as u32);
        self.ctx.viewport(0,0,s.0,s.1);
        elel.set_attribute("style",&format!("width: {}px; height: {}px",s.0,s.1)).ok();
    }
    
    pub fn get_available_size(&self) -> Dot<i32,i32> {
        let ws = domutil::window_space(&self.canv_el.parent_node().unwrap().try_into().unwrap());
        let mut size = domutil::size(&self.canv_el.parent_node().unwrap().try_into().unwrap());
        // TODO left/top/right
        let rb = ws.far_offset();
        if rb.1 < 0 {
            // off the bottom, fix
            size.1 += rb.1
        }
        size
    }
}