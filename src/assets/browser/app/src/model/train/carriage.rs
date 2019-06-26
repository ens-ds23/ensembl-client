use std::sync::{ Arc, Mutex };

use composit::Leaf;
use composit::{ StateManager };
use model::driver::{ Printer, PrinterManager };
use drivers::zmenu::{ ZMenuLeaf, ZMenuLeafSet };
use super::Traveller;

pub struct Carriage {
    pm: PrinterManager,
    travellers: Vec<Traveller>,
    known_done: bool,
    needs_rebuild: bool,
    leaf: Leaf
}

impl Carriage {
    pub(in super) fn new(pm: &PrinterManager,leaf: &Leaf) -> Carriage {
        let mut out = Carriage {
            pm: pm.clone(),
            travellers: Vec::<Traveller>::new(),
            known_done: false,
            needs_rebuild: false,
            leaf: leaf.clone()
        };
        out.pm.add_leaf(leaf);
        out
    }
    
    pub fn get_leaf(&self) -> &Leaf { &self.leaf }
    
    pub(in super) fn set_needs_refresh(&mut self) {
        self.needs_rebuild = true;
    }
    
    pub(in super) fn add_traveller(&mut self, traveller: Traveller) {
        self.travellers.push(traveller);
    }
        
    pub(in super) fn is_done(&mut self) -> bool {
        if self.known_done { return true; }
        for c in &self.travellers {
            if !c.is_done() { return false; }
        }
        self.known_done = true;
        self.needs_rebuild = true;
        return true;
    }
    
    pub fn update_state(&mut self, oom: &StateManager) {
        let mut redo = false;
        for t in &mut self.travellers {
            if t.update_state(oom) {
                redo = true;
            }
        }
        if redo {
            self.needs_rebuild = true;
        }
    }
    
    fn build_zmenu(&self, zml: &mut ZMenuLeaf) {
        for t in &self.travellers {
            t.build_zmenu(zml);
        }
        zml.redrawn();
    }
    
    pub fn redraw_where_needed(&mut self, printer: &mut Printer, zmls: &mut ZMenuLeafSet) {
        let mut zml = ZMenuLeaf::new(&self.leaf);
        if self.needs_rebuild {
            self.needs_rebuild = false;
            printer.redraw_carriage(&self.leaf);
            self.build_zmenu(&mut zml);
        }
        zmls.register_leaf(zml);
    }
}

impl Drop for Carriage {
    fn drop(&mut self) {
        self.travellers.clear(); // Triggers drop which informs printer
        self.pm.remove_leaf(&self.leaf);
    }
}
