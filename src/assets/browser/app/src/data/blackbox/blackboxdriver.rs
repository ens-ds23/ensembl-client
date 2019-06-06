use std::sync::{ Arc, Mutex };

use super::BlackBoxState;

pub trait BlackBoxDriverImpl {
    fn tick(&mut self, state: &mut BlackBoxState, t: f64) -> bool { false }    
}

#[derive(Clone)]
pub struct BlackBoxDriver(Arc<Mutex<Box<BlackBoxDriverImpl>>>);

impl BlackBoxDriver {
    pub fn new(driver: Box<BlackBoxDriverImpl>) -> BlackBoxDriver {
        BlackBoxDriver(Arc::new(Mutex::new(driver)))
    }
    
    pub fn tick(&mut self, state: &mut BlackBoxState, t: f64) -> bool {
        self.0.lock().unwrap().tick(state,t)
    }
}
