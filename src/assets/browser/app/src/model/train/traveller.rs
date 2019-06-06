use std::fmt;

use composit::{ Leaf, ActiveSource };
use composit::{ StateManager };
use model::driver::{ PrinterManager, SourceResponse };

pub struct Traveller {
    pm: PrinterManager,
    comp: ActiveSource,
    prev_value: bool,
    cur_value: bool,
    srr: Option<Box<SourceResponse>>,
    part: Option<String>,
    leaf: Leaf
}

impl Traveller {
    pub fn new(pm: &PrinterManager, comp: ActiveSource, part: &Option<String>, leaf: &Leaf, srr: Box<SourceResponse>) -> Traveller {
        Traveller {
            pm: pm.clone(),
            prev_value: false,
            cur_value: false,
            leaf: leaf.clone(),
            part: part.clone(),
            comp,
            srr: Some(srr)
        }
    }
    
    pub(in super) fn update_state(&mut self, m: &StateManager) -> bool {
        self.prev_value = self.cur_value;
        self.cur_value = self.comp.is_on(m,&self.part);
        self.srr.as_ref().unwrap().set_state(self.cur_value);
        self.prev_value != self.cur_value
    }

    pub(in super) fn is_done(&self) -> bool { 
        return self.srr.as_ref().unwrap().check();
    }
    
    pub fn is_on(&self) -> bool { self.cur_value }
    
    pub fn get_leaf(&self) -> &Leaf { &self.leaf }
}

impl Drop for Traveller {
    fn drop(&mut self) {
        self.srr.take().unwrap().destroy();
    }
}


impl fmt::Debug for Traveller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?}:{:?}({:?})",self.comp,self.leaf,self.part)
    }
}
