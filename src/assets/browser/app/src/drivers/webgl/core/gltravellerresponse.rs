use std::cell::RefCell;
use std::fmt;
use std::hash::{ Hash, Hasher };
use std::rc::Rc;

use super::{ GLDrawing, GLProgInstances, GLPrinter };
use super::super::drawing::CarriageCanvases;
use composit::Leaf;
use model::train::{ TravellerResponse, TravellerResponseData };
use composit::source::SourceResponse;
use drivers::zmenu::ZMenuLeaf;

#[derive(Clone)]
pub struct GLTravellerResponse {
    printer: GLPrinter,
    idx: usize,
    dr: Rc<RefCell<Option<GLDrawing>>>,
    pending_refresh: Rc<RefCell<bool>>,
    state: Rc<RefCell<bool>>,
    leaf: Leaf
}

impl PartialEq for GLTravellerResponse {
    fn eq(&self, other: &GLTravellerResponse) -> bool {
        self.idx == other.idx
    }
}
impl Eq for GLTravellerResponse {}

impl Hash for GLTravellerResponse {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}

impl GLTravellerResponse {    
    /* train/partyresponses */
    pub(in super) fn new(printer: &GLPrinter, idx: usize, leaf: &Leaf) -> GLTravellerResponse {
        GLTravellerResponse {
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
        let mut dr = self.dr.borrow_mut();
        if dr.is_some() {
            dr.as_mut().unwrap().redraw(cc);
        }
    }
    
    pub fn redraw_objects(&self, e: &mut GLProgInstances) {
        let mut dr = self.dr.borrow_mut();
        if dr.is_some() && *self.state.borrow() {
            dr.as_mut().unwrap().into_objects(e);
        }
    }    
}

impl TravellerResponse for GLTravellerResponse {
    /* train/traveller */
    fn check(&self) -> bool {
        self.dr.borrow().is_some()
    }

    fn set_state(&self, state: bool) {
        *self.state.borrow_mut() = state;
    }

    /* train/partyresponses */
    fn set_response(&mut self, result: TravellerResponseData) {
        *self.dr.borrow_mut() = Some(GLDrawing::new(result));
    }

    fn refresh(&mut self) {
        *self.pending_refresh.borrow_mut() = true;
    }
    
    fn destroy(&mut self) {
        let mut p = self.printer.clone();
        p.destroy_partial(self);
    }
    
    fn source_response_clone(&self) -> Box<TravellerResponse> {
        Box::new(self.clone())
    }
}

impl fmt::Debug for GLTravellerResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?}[{}]",self.leaf,self.idx)
    }
}
