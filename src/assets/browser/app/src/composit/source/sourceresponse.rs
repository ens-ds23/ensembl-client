use std::collections::HashMap;

use drivers::zmenu::ZMenuLeaf;
use model::driver::{ Printer, PrinterManager };
use model::train::{ Traveller, TravellerResponse, TravellerResponseData };
use composit::Leaf;

pub struct SourceResponse {
    leaf: Leaf,
    travellers: HashMap<Option<String>,Traveller>
}

impl SourceResponse {
    pub fn new(pm: &mut PrinterManager, leaf: &Leaf, tt: &mut Vec<Traveller>) -> SourceResponse {
        let mut travs = HashMap::new();
        for t in tt.iter() {
            travs.insert(t.get_part().clone(),t.clone());
        }
        let mut out = SourceResponse {
            leaf: leaf.clone(),
            travellers: travs
        };
        for t in tt {
            t.set_visuals(pm.make_traveller_response(&leaf));
        }
        out
    }
    
    pub fn update_zml<F>(&mut self, part: &Option<String>, cb: F) where F: FnOnce(&mut ZMenuLeaf) {
        self.travellers.get_mut(part).unwrap().update_zml(cb);
    }
    
    pub fn update_data<F>(&mut self,  part: &Option<String>, cb: F) where F: FnOnce(&mut TravellerResponseData) {
        self.travellers.get_mut(part).unwrap().update_data(cb);
    }
        
    pub fn done(&mut self) {
        for (_,t) in &mut self.travellers {
            t.set_response();
        }
    }
}
