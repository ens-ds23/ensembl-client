use std::cell::RefCell;
use std::collections::{ HashMap, HashSet };
use std::rc::Rc;

use stdweb::unstable::TryInto;
use stdweb::web::{ HtmlElement, Element, INode, IElement };

use super::{ Programs, CarriagePrinter, GLSourceResponse };
use composit::{ Compositor, StateManager, Leaf, Stage };
use model::driver::Printer;
use model::train::Train;
use drawing::{ AllCanvasAllocator };
use dom::domutil;
use types::{ Dot };

use dom::webgl::WebGLRenderingContext as glctx;
use stdweb::web::html_element::{
    CanvasElement
};

pub struct WebGLTrainPrinter{}

impl WebGLTrainPrinter {
    pub fn new() -> WebGLTrainPrinter {
        WebGLTrainPrinter {}
    }
    
    fn execute(&mut self, printer: &mut WebGLPrinterBase, leafs: &Vec<Leaf>) {
        for pt in &printer.base_progs.order {
            for leaf in leafs.iter() {
                let lp = &mut expect!(printer.lp.get_mut(&leaf)); 
                lp.execute(&pt);
            }
        }
    }
    
    fn prepare(&mut self, printer: &mut WebGLPrinterBase, stage: &Stage, oom: &StateManager,
                     train: &mut Train, opacity: f32) {
        for carriage in train.get_carriages() {
            let leaf = carriage.get_leaf().clone();
            if let Some(lp) = &mut printer.lp.get_mut(&leaf) {
                lp.prepare(oom,carriage,&mut printer.acm,stage,opacity);
            }
        }
    }
}

pub struct WebGLPrinterBase {
    sridx: usize,
    canv_el: HtmlElement,
    ctx: Rc<glctx>,
    base_progs: Programs,
    acm: AllCanvasAllocator,
    lp: HashMap<Leaf,CarriagePrinter>,
    current: HashSet<Leaf>
}

impl WebGLPrinterBase {
    pub fn new(canv_el: &HtmlElement) -> WebGLPrinterBase {
        let canvas = canv_el.clone().try_into().unwrap();
        let ctx: glctx = domutil::get_context(&canvas);
        ctx.clear_color(1.0,1.0,1.0,1.0);
        ctx.clear(glctx::COLOR_BUFFER_BIT  | glctx::DEPTH_BUFFER_BIT);
        let ctx_rc = Rc::new(ctx);
        let progs = Programs::new(&ctx_rc);
        let acm = AllCanvasAllocator::new(".bpane-container .managedcanvasholder");
        WebGLPrinterBase {
            sridx: 0,
            canv_el: canv_el.clone(),
            acm, ctx: ctx_rc,
            base_progs: progs,
            lp: HashMap::<Leaf,CarriagePrinter>::new(),
            current: HashSet::<Leaf>::new()
        }
    }

    pub fn add_leaf(&mut self, leaf: &Leaf) {
        let progs = self.base_progs.clean_instance();
        self.lp.insert(leaf.clone(),CarriagePrinter::new(&mut self.acm,&leaf,&progs,&self.ctx));
    }
    
    pub fn remove_leaf(&mut self, leaf: &Leaf) {
        if let Some(mut lp) = self.lp.remove(&leaf) {
            lp.destroy(&mut self.acm);
        }
        self.current.remove(leaf);
    }
    
    fn set_current(&mut self, leaf: &Leaf) {
        self.current.insert(leaf.clone());
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
    
    fn set_size(&mut self, s: Dot<i32,i32>) {
        let elel: Element =  self.canv_el.clone().into();
        let elc : CanvasElement = elel.clone().try_into().unwrap();
        elc.set_width(s.0 as u32);
        elc.set_height(s.1 as u32);
        self.ctx.viewport(0,0,s.0,s.1);
        elel.set_attribute("style",&format!("width: {}px; height: {}px",s.0,s.1)).ok();
    }
    
    fn get_available_size(&self) -> Dot<i32,i32> {
        let ws = domutil::window_space(&self.canv_el.parent_node().unwrap().try_into().unwrap());
        let mut size = domutil::size(&self.canv_el.parent_node().unwrap().try_into().unwrap());
        // TODO left/top/right
        let rb = ws.far_offset();
        if rb.1 < 0 {
            // off the bottom, fix
            size.1 += rb.1
        }
        /* Rendering can go fuzzy if available size not multiple of 4 */
        size.0 = ((size.0+3)/4)*4;
        size.1 = ((size.1+3)/4)*4;
        size
    }

    fn destroy(&mut self) {
        for (_i,mut lp) in &mut self.lp {
            lp.destroy(&mut self.acm);
        }
        self.acm.finish();
    }    

    fn make_partial(&mut self, leaf: &Leaf) -> GLSourceResponse {
        let idx = self.sridx;
        self.sridx += 1;
        let sr = GLSourceResponse::new(idx,leaf);
        if let Some(cp) = self.lp.get_mut(leaf) {
            cp.new_sr(&sr);
        }
        sr
    }
    
    fn destroy_partial(&mut self, sr: GLSourceResponse) {
        let leaf = sr.get_leaf().clone();
        if let Some(cp) = self.lp.get_mut(&leaf) {
            cp.remove_sr(sr);
        }        
    }
}

#[derive(Clone)]
pub struct WebGLPrinter {
    base: Rc<RefCell<WebGLPrinterBase>>
}

impl WebGLPrinter {
    pub fn new(canv_el: &HtmlElement) -> WebGLPrinter {
        WebGLPrinter {
            base: Rc::new(RefCell::new(WebGLPrinterBase::new(canv_el)))
        }
    }
}

impl Printer for WebGLPrinter {
    fn print(&mut self, stage: &Stage, oom: &StateManager, compo: &mut Compositor) {
        let prop = compo.get_prop_trans();
        if let Some(train) = compo.get_current_train(true) {
            let mut tp = WebGLTrainPrinter::new();
            tp.prepare(&mut self.base.borrow_mut(),stage,oom,train,1.-prop);
        }
        if let Some(train) = compo.get_transition_train(true) {
            let mut tp = WebGLTrainPrinter::new();
            tp.prepare(&mut self.base.borrow_mut(),stage,oom,train,prop);
        }
        self.base.borrow_mut().prepare_all();
        if let Some(train) = compo.get_transition_train(true) {
            let mut tp = WebGLTrainPrinter::new();
            tp.execute(&mut self.base.borrow_mut(),&train.leafs());
        }
        if let Some(train) = compo.get_current_train(true) {
            let mut tp = WebGLTrainPrinter::new();
            tp.execute(&mut self.base.borrow_mut(),&train.leafs());
        }
    }

    fn destroy(&mut self) {
        self.base.borrow_mut().destroy();
    }

    fn set_size(&mut self, s: Dot<i32,i32>) {
        self.base.borrow_mut().set_size(s);
    }
    
    fn get_available_size(&self) -> Dot<i32,i32> {
        self.base.borrow().get_available_size()
    }

    fn add_leaf(&mut self, leaf: &Leaf) {
        self.base.borrow_mut().add_leaf(leaf);
    }
    
    fn remove_leaf(&mut self, leaf: &Leaf) {
        self.base.borrow_mut().remove_leaf(leaf);
    }
    
    fn set_current(&mut self, leaf: &Leaf) {
        self.base.borrow_mut().set_current(leaf);
    }
    
    fn make_partial(&mut self, leaf: &Leaf) -> GLSourceResponse {
        self.base.borrow_mut().make_partial(leaf)
    }
    
    fn destroy_partial(&mut self, sr: GLSourceResponse) {
        self.base.borrow_mut().destroy_partial(sr);
    }
}
