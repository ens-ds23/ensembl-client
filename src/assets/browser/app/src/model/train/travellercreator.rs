use std::collections::HashMap;
use std::collections::hash_map::Entry;

use model::driver::PrinterManager;
use super::{ PartyResponses, Traveller };

const MAX_PENDING : i32 = 0;
const CACHE_SIZE : usize = 10;

use composit::{
    ActiveSource, Leaf
};

pub struct TravellerCreator {
    pm: PrinterManager,
    components: HashMap<String,ActiveSource>
}

impl TravellerCreator {
    /* compositor/new() */
    pub fn new(pm: &PrinterManager) -> TravellerCreator {
        TravellerCreator {
            pm: pm.clone(),
            components: HashMap::<String,ActiveSource>::new()
        }
    }
       
    /* compositor/add_component() */
    pub fn add_source(&mut self, c: ActiveSource) {
        let name = c.get_name().to_string();
        if let Entry::Vacant(e) = self.components.entry(name) {
            e.insert(c);
        }
    }

    /* compositor/tick() */
    pub fn remove_source(&mut self, k: &str) {
        self.components.remove(k);
    }
    
    /* train/add_component() */
    pub fn make_party(&mut self, c: &ActiveSource, leaf: &Leaf) -> Vec<Traveller> {
        let party = PartyResponses::new(&self.pm,&c.list_parts(),leaf);
        let source = c.get_source().clone();
        let out = c.make_party(&self.pm,&party,leaf);
        source.populate(c,party,leaf);
        out
    }
    
    /* train/manage_leafs() */
    pub fn make_leaf_parties(&mut self, leaf: Leaf) -> Vec<Traveller> {
        let mut lcomps = Vec::<Traveller>::new();
        debug!("redraw","make_carriages {:?}",leaf);
        let comps : Vec<ActiveSource> = self.components.values().cloned().collect();
        for c in &comps {
            lcomps.append(&mut self.make_party(c,&leaf));
        }
        lcomps
    }    
}
