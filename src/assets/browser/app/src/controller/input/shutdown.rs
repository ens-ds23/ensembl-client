use std::sync::{ Arc, Mutex };

use controller::global::Global;

use dom::domutil;
use dom::event::{ EventListener, EventType, EventData, EventControl, Target };

pub struct ShutdownEventListener {
    g: Global
}

impl ShutdownEventListener {
    pub fn new(g: &Global) -> ShutdownEventListener {
        ShutdownEventListener {
            g: g.clone()
        }
    }    
}

impl EventListener<()> for ShutdownEventListener {
    fn receive(&mut self, _el: &Target,  e: &EventData, _idx: &()) {
        match e {
            EventData::GenericEvent(EventType::UnloadEvent,_) => {
                self.g.destroy();
            },
            _ => ()
        }
    }
}

pub fn register_shutdown_events(g: &Global) {
    let uel = ShutdownEventListener::new(g);
    let mut ec_start = EventControl::new(Box::new(uel),());
    ec_start.add_event(EventType::UnloadEvent);
    ec_start.add_element(&domutil::query_select("body"),());
}
