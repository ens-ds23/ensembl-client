use std::cell::RefCell;
use std::collections::{ HashMap, HashSet };
use std::rc::Rc;

use stdweb::unstable::TryInto;
use stdweb::web::{ HtmlElement, Element, INode, IElement };

use super::{ GLProgs, GLCarriagePrinter, GLTravellerResponse };
use composit::{ Compositor, Leaf, Stage };
use model::driver::Printer;
use model::train::{ Train, TravellerResponse };
use super::super::drawing::{ AllCanvasAllocator };
use dom::domutil;
use types::{ Dot };

use dom::webgl::WebGLRenderingContext as glctx;
use stdweb::web::html_element::{
    CanvasElement
};

use drivers::zmenu::{ ZMenuRegistry, ZMenuLeafSet, ZMenuLeaf };

pub struct WebGLTrainPrinter{}

impl WebGLTrainPrinter {
    pub fn new() -> WebGLTrainPrinter {
        WebGLTrainPrinter {}
    }
    
    fn execute(&mut self, printer: &mut GLPrinterBase, leafs: &Vec<Leaf>) {
        for pt in &printer.base_progs.order {
            for leaf in leafs.iter() {
                if let Some(ref mut lp) = printer.lp.get_mut(&leaf) {
                    lp.execute(&pt);
                }
            }
        }
    }
    
    fn contextualize(&mut self, printer: &mut GLPrinterBase, stage: &Stage,
                     train: &mut Train, opacity: f32) {
        for carriage in train.get_carriages() {
            let leaf = carriage.get_leaf();
            if let Some(carriage_printer) = &mut printer.lp.get_mut(&leaf) {
                carriage_printer.set_context(stage,opacity);
            }
        }
    }
}

pub struct GLPrinterBase {
    sridx: usize,
    canv_el: HtmlElement,
    ctx: Rc<glctx>,
    base_progs: GLProgs,
    acm: AllCanvasAllocator,
    lp: HashMap<Leaf,GLCarriagePrinter>,
    current: HashSet<Leaf>,
    new_size: Option<Dot<f64,f64>>,
    settled_size: Option<Dot<f64,f64>>,
    round_size: bool,
}

impl GLPrinterBase {
    pub fn new(canv_el: &HtmlElement) -> GLPrinterBase {
        let canvas = canv_el.clone().try_into().unwrap();
        let ctx: glctx = domutil::get_context(&canvas);
        ctx.clear_color(1.0,1.0,1.0,1.0);
        ctx.clear(glctx::COLOR_BUFFER_BIT  | glctx::DEPTH_BUFFER_BIT);
        let ctx_rc = Rc::new(ctx);
        let progs = GLProgs::new(&ctx_rc);
        let acm = AllCanvasAllocator::new(".bpane-container .managedcanvasholder");
        GLPrinterBase {
            sridx: 0,
            canv_el: canv_el.clone(),
            acm, ctx: ctx_rc,
            base_progs: progs,
            lp: HashMap::<Leaf,GLCarriagePrinter>::new(),
            current: HashSet::<Leaf>::new(),
            new_size: None,
            settled_size: None,
            round_size: true,
        }
    }

    pub fn redraw_carriage(&mut self, leaf: &Leaf) {
        if let Some(carriage_printer) = &mut self.lp.get_mut(&leaf) {
            carriage_printer.redraw(&mut self.acm);
        }
    }

    pub fn add_leaf(&mut self, leaf: &Leaf) {
        self.lp.insert(leaf.clone(),GLCarriagePrinter::new(&leaf,&self.base_progs,&self.ctx));
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
        if let Some(new_size) = self.new_size.take() {
            self.set_size(new_size);
        }
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
    
    fn set_size(&mut self, s: Dot<f64,f64>) {
        /* Rendering can go fuzzy if available size not multiple of 4 */
        let vp_sz = Dot(s.0.round() as i32,s.1.round() as i32);
        let mut css = "width: 100%; height: 100%".to_string();
        if self.round_size {
            css = format!("width: {}px; height: {}px",vp_sz.0,vp_sz.1);
        }
        self.round_size = false;
        let elel: Element =  self.canv_el.clone().into();
        let elc : CanvasElement = elel.clone().try_into().unwrap();
        elc.set_width(vp_sz.0 as u32);
        elc.set_height(vp_sz.1 as u32);
        self.ctx.viewport(0,0,vp_sz.0 as i32,vp_sz.1 as i32);
        elel.set_attribute("style",&css).ok();
    }
    
    fn get_available_size(&self) -> Dot<f64,f64> {
        let ws = domutil::window_space(&self.canv_el.parent_node().unwrap().try_into().unwrap());
        let mut size = domutil::size(&self.canv_el.parent_node().unwrap().try_into().unwrap());
        // TODO left/top/right
        let rb = ws.far_offset();
        if rb.1 < 0. {
            // off the bottom, fix
            size.1 += rb.1
        }
        size
    }

    fn settle(&mut self) {
        self.round_size = true;
        self.set_size(self.get_available_size());
    }

    fn destroy(&mut self) {
        for (_i,lp) in &mut self.lp {
            lp.destroy(&mut self.acm);
        }
        self.acm.finish();
        let gl : Option<glctx> = ok!(
            js! { return @{self.canv_el.as_ref()}.getContext("webgl"); }.try_into()
        );
        if let Some(gl) = gl {
            js! {
                var x = @{gl.as_ref()}.getExtension("WEBGL_lose_context");
                if(x) x.loseContext();
            }
        }
    }    

    fn make_traveller_response(&mut self, pref: &GLPrinter, leaf: &Leaf) -> Box<TravellerResponse> {
        let idx = self.sridx;
        self.sridx += 1;
        let sr = GLTravellerResponse::new(pref,idx,leaf);
        if let Some(cp) = self.lp.get_mut(leaf) {
            cp.new_sr(&sr);
        }
        Box::new(sr)
    }
    
    fn destroy_partial(&mut self, sr: &mut GLTravellerResponse) {
        let leaf = sr.get_leaf().clone();
        if let Some(cp) = self.lp.get_mut(&leaf) {
            cp.remove_sr(sr);
        }        
    }
}

#[derive(Clone)]
pub struct GLPrinter {
    base: Rc<RefCell<GLPrinterBase>>
}

impl GLPrinter {
    pub fn new(canv_el: &HtmlElement) -> GLPrinter {
        GLPrinter {
            base: Rc::new(RefCell::new(GLPrinterBase::new(canv_el)))
        }
    }
    
    pub(in super) fn destroy_partial(&mut self, sr: &mut GLTravellerResponse) {
        self.base.borrow_mut().destroy_partial(sr);
    }
}

impl Printer for GLPrinter {
    fn print(&mut self, stage: &Stage, compo: &mut Compositor) {
        compo.redraw_where_needed(self);
        let prop = compo.get_prop_trans();
        if let Some(train) = compo.get_current_train() {
            let mut tp = WebGLTrainPrinter::new();
            tp.contextualize(&mut self.base.borrow_mut(),stage,train,1.-prop);
        }
        if let Some(train) = compo.get_transition_train() {
            let mut tp = WebGLTrainPrinter::new();
            tp.contextualize(&mut self.base.borrow_mut(),stage,train,prop);
        }
        self.base.borrow_mut().prepare_all();
        if let Some(train) = compo.get_transition_train() {
            let mut tp = WebGLTrainPrinter::new();
            tp.execute(&mut self.base.borrow_mut(),&train.leafs());
        }
        if let Some(train) = compo.get_current_train() {
            let mut tp = WebGLTrainPrinter::new();
            tp.execute(&mut self.base.borrow_mut(),&train.leafs());
        }
    }

    fn destroy(&mut self) {
        self.base.borrow_mut().destroy();
    }

    fn redraw_carriage(&mut self, leaf: &Leaf) {
        self.base.borrow_mut().redraw_carriage(leaf);
    }

    fn set_size(&mut self, s: Dot<f64,f64>) {
        self.base.borrow_mut().new_size = Some(s);
        self.base.borrow_mut().settled_size = Some(s);
    }
    
    fn settle(&mut self) {
        self.base.borrow_mut().settle();
    }
    
    fn get_available_size(&self) -> Dot<f64,f64> {
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
    
    fn make_traveller_response(&mut self, leaf: &Leaf) -> Box<TravellerResponse> {
        let twin = self.clone();
        self.base.borrow_mut().make_traveller_response(&twin,leaf)
    }      
}
