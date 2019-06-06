use std::cell::RefCell;
use std::fmt;
use std::hash::{ Hash, Hasher };
use std::rc::Rc;

use super::{ GLDrawing, GLProgInstances, GLPrinter };
use super::super::drawing::CarriageCanvases;
use composit::{ Leaf, SourceResponseData };
use model::driver::SourceResponse;

#[derive(Clone)]
pub struct GLSourceResponse {
    printer: GLPrinter,
    idx: usize,
    dr: Rc<RefCell<Option<GLDrawing>>>,
    pending_refresh: Rc<RefCell<bool>>,
    state: Rc<RefCell<bool>>,
    leaf: Leaf
}

impl PartialEq for GLSourceResponse {
    fn eq(&self, other: &GLSourceResponse) -> bool {
        self.idx == other.idx
    }
}
impl Eq for GLSourceResponse {}

impl Hash for GLSourceResponse {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}

impl GLSourceResponse {    
    /* train/partyresponses */
    pub(in super) fn new(printer: &GLPrinter, idx: usize, leaf: &Leaf) -> GLSourceResponse {
        GLSourceResponse {
            printer: printer.clone(),
            idx,
            dr: Rc::new(RefCell::new(None)),
            pending_refresh: Rc::new(RefCell::new(false)), // XXX unused
            state: Rc::new(RefCell::new(false)),
            leaf: leaf.clone()
        }
    }
        
    pub fn get_leaf(&self) -> &Leaf { &self.leaf }
    
    pub fn redraw_drawings(&self, cc: &mut CarriageCanvases) {
        //console!("drawings {:?}",self.leaf);
        let mut dr = self.dr.borrow_mut();
        if dr.is_some() {
            dr.as_mut().unwrap().redraw(cc);
        }
    }
    
    pub fn redraw_objects(&self, e: &mut GLProgInstances) {
        let mut dr = self.dr.borrow_mut();
        if dr.is_some() && *self.state.borrow() {
            //console!("objects {:?}",self.leaf);
            dr.as_mut().unwrap().into_objects(e);
        }
    }
}

impl SourceResponse for GLSourceResponse {
    /* train/traveller */
    fn check(&self) -> bool {
        self.dr.borrow().is_some()
    }

    fn set_state(&self, state: bool) {
        *self.state.borrow_mut() = state;
    }

    /* train/partyresponses */
    fn set(&mut self, result: SourceResponseData) {
        *self.dr.borrow_mut() = Some(GLDrawing::new(result));
    }

    fn refresh(&mut self) {
        *self.pending_refresh.borrow_mut() = true;
    }
    
    fn destroy(&mut self) {
        let mut p = self.printer.clone();
        p.destroy_partial(self);
    }
    
    fn source_response_clone(&self) -> Box<SourceResponse> {
        Box::new(self.clone())
    }
}

impl fmt::Debug for GLSourceResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?}[{}]",self.leaf,self.idx)
    }
}
