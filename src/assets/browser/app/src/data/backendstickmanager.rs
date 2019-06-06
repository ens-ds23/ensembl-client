use std::collections::HashMap;

use composit::{ Stick, StickManager };
use super::BackendConfig;

pub struct BackendStickManager {
    sticks: HashMap<String,Stick>
}

impl BackendStickManager {
    pub fn new(config: &BackendConfig) -> BackendStickManager {
        let out = BackendStickManager {
            sticks: config.get_sticks().clone()
        };
        out
    }    
}

impl StickManager for BackendStickManager {
    fn get_stick(&mut self, name: &str) -> Option<Stick> {
        self.sticks.get(name).cloned()
    }    
}
