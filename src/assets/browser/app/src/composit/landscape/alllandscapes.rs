use std::cell::RefCell;
use std::rc::Rc;

use util::ValueStore;

use super::landscape::Landscape;

pub struct AllLandscapesImpl {
    vs: ValueStore<Landscape>,
}

impl AllLandscapesImpl {
    fn new() -> AllLandscapesImpl {
        AllLandscapesImpl {
            vs: ValueStore::<Landscape>::new(),
        }
    }
    
    fn allocate(&mut self, name: &str) -> usize {
        let out = self.vs.store(Landscape::new(name));
        out
    }
    
    fn with<F,G>(&mut self, lid: usize, cb: F) -> Option<G>
            where F: FnOnce(&mut Landscape) -> G {
        self.vs.get_mut(lid).map(|ls| cb(ls))
    }

    pub fn every<F,G>(&mut self, mut cb: F) -> Vec<Option<G>>
            where F: FnMut(usize, &mut Landscape) -> G {
        let mut out = Vec::<Option<G>>::new();
        let lids : Vec<usize> = self.vs.every().collect();
        for lid in lids {
            out.push(self.vs.get_mut(lid).map(|ls| cb(lid,ls)));
        }
        out
    }
    
    pub fn get_low_watermark(&self) -> i32 {
        let mut max = 0;
        for lid in self.vs.every() {
            if let Some(ls) = self.vs.get(lid) {
                let wm = ls.get_low_watermark().unwrap_or(0);
                max = max.max(wm)
            }
        }
        max
    }
}

#[derive(Clone)]
pub struct AllLandscapes(Rc<RefCell<AllLandscapesImpl>>);

impl AllLandscapes {
    pub fn new() -> AllLandscapes {
        AllLandscapes(Rc::new(RefCell::new(AllLandscapesImpl::new())))
    }
    
    pub fn allocate(&mut self, name: &str) -> usize {
        let lid = self.0.borrow_mut().allocate(name);
        self.with(lid,|ls| ls.set_lid(lid));
        lid
    }
    
    pub fn with<F,G>(&mut self, lid: usize, cb: F) -> Option<G>
            where F: FnOnce(&mut Landscape) -> G {
        self.0.borrow_mut().with(lid,cb)
    }
    
    pub fn every<F,G>(&mut self, cb: F) -> Vec<Option<G>>
            where F: FnMut(usize, &mut Landscape) -> G {
        self.0.borrow_mut().every(cb)
    }
    
    pub fn get_low_watermark(&self) -> i32 {
        self.0.borrow().get_low_watermark()
    }
}
