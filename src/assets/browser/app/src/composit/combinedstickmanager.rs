use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use composit::{ Stick, StickManager };
use data::{ BackendConfigBootstrap, BackendStickManager };

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
        if let Some(stick) = self.internal.get(name) {
            Some(stick).cloned()
        } else {        
            self.backend.get_stick(name)
        }
    }
}