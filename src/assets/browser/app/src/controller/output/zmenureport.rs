use std::sync::{ Arc, Mutex };

use serde_json::Value as JSONValue;

use controller::global::AppRunner;
use controller::output::OutputAction;
use types::Dot;

struct ZMenuEventQueue {
    pending: Vec<JSONValue>
}

impl ZMenuEventQueue {
    pub fn new() -> ZMenuEventQueue {
        ZMenuEventQueue {
            pending: Vec::new()
        }
    }
    
    fn add_report(&mut self, payload: JSONValue) {
        self.pending.push(payload);
    }
    
    fn get_reports(&mut self) -> Vec<JSONValue> {
        self.pending.drain(..).collect()
    }    
}

pub struct ZMenuReports {
    activated: Option<String>,
    queue: Arc<Mutex<ZMenuEventQueue>>
}

impl ZMenuReports {
    pub fn get_reports(&self) -> Vec<JSONValue> {
        self.queue.lock().unwrap().get_reports()
    }
    
    pub fn add_activate(&mut self, id: &str, pos: Dot<i32,i32>, payload: JSONValue) {
        self.deactivate();
        console!("add {}",payload.to_string());
        self.activated = Some(id.to_string());
        unwrap!(self.queue.lock()).add_report(json!({
            "action": "create_zmenu",
            "id": id,
            "content": [payload],
            "anchor_coordinates": { "x": pos.0, "y": pos.1 }
        }));
    }
    
    pub fn deactivate(&mut self) {
        if let Some(ref id) = self.activated {
            unwrap!(self.queue.lock()).add_report(json!({
                "action": "destroy_zmenu",
                "id": id
            }));
            self.activated = None;
        }
    }
    
    pub fn new(ar: &mut AppRunner) -> ZMenuReports {
        let mut out = ZMenuReports{
            queue: Arc::new(Mutex::new(ZMenuEventQueue::new())),
            activated: None
        };
        let queue = out.queue.clone();
        ar.add_timer("zmenu-report", move |_app,t,sr| {
            let mut reports = unwrap!(queue.lock()).get_reports();
            if reports.len() != 0 {
                reports.drain(..).map(|report| {
                    OutputAction::SendCustomEvent("bpane-zmenu".to_string(),report)
                }).collect()
            } else {
                sr.unproductive();
                vec![]
            }
        },4);
        out
    }
}
