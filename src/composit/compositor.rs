use std::collections::HashMap;

use composit::{ LeafComponent, StateManager, Component, Leaf };
use composit::state::ComponentRedo;

pub struct Compositor {
    idx: u32,
    components: HashMap<u32,Component>,
    leafcomps: HashMap<Leaf,HashMap<u32,LeafComponent>>
}

pub struct ComponentRemover(u32);

impl Compositor {
    pub fn new() -> Compositor {
        Compositor {
            components: HashMap::<u32,Component>::new(),
            leafcomps: HashMap::<Leaf,HashMap<u32,LeafComponent>>::new(),
            idx: 0
        }
    }

    pub fn leafs(&self) -> Vec<Leaf> {
        self.leafcomps.keys().map(|s| s.clone()).collect()
    }

    pub fn add_leaf(&mut self, leaf: Leaf) {
        let mut lcc = HashMap::<u32,LeafComponent>::new();
        for (k,c) in &self.components {
            lcc.insert(*k,c.make_leafcomp(&leaf));
        }
        self.leafcomps.insert(leaf,lcc);
    }
    
    pub fn remove_leaf(&mut self, leaf: &Leaf) {
        self.leafcomps.remove(leaf);
    }
    
    pub fn get_components(&mut self, leaf: &Leaf) -> Option<Vec<&mut LeafComponent>> {
        self.leafcomps.get_mut(leaf).map(|s| s.values_mut().collect())
    }
    
    pub fn add_component(&mut self, c: Component) -> ComponentRemover {
        self.idx += 1;
        for (ref mut leaf,ref mut lcc) in &mut self.leafcomps {
            lcc.insert(self.idx,c.make_leafcomp(&leaf));
        }
        self.components.insert(self.idx,c);
        ComponentRemover(self.idx)
    }

    pub fn remove(&mut self, k: ComponentRemover) {
        self.components.remove(&k.0);
    }
    
    pub fn calc_level(&mut self, leaf: &Leaf, oom: &StateManager) -> ComponentRedo {
        let mut redo = ComponentRedo::None;
        if let Some(ref mut lcc) = self.leafcomps.get_mut(leaf) {
            for c in lcc.values_mut() {
                redo = redo | c.update_state(oom);
            }
        }
        redo
    }
}
