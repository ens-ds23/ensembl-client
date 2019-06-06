use std::collections::HashMap;

use composit::{ Stick, StickManager };
use data::BackendStickManager;

pub struct CombinedStickManager {
    backend: BackendStickManager,
    internal: HashMap<String,Stick>
}

impl CombinedStickManager {
    pub fn new(backend: BackendStickManager) -> CombinedStickManager {
        CombinedStickManager {
            backend,
            internal: HashMap::<String,Stick>::new()
        }
    }
    
    pub fn add_internal_stick(&mut self, name: &str, stick: Stick) {
        self.internal.insert(name.to_string(),stick);
    }
}

impl StickManager for CombinedStickManager {
    fn get_stick(&mut self, name: &str) -> Option<Stick> {
        console!("csm: {:?} ({:?})",name,self.internal.get(name).is_some());
        if let Some(stick) = self.internal.get(name) {
            Some(stick).cloned()
        } else {
            self.backend.get_stick(name)
        }
    }
}
