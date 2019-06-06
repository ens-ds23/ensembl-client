use std::collections::{ HashMap, HashSet };
use std::sync::{ Arc, Mutex };

use serde_json::Value as SerdeValue;
use serde_json::Map as SerdeMap;
use stdweb::unstable::TryInto;

use dom::domutil::browser_time;
use util::get_instance_id;
use super::bbreportstream::{ BlackBoxReport, BlackBoxReportStream };

pub struct BlackBoxStateImpl {
    pending: HashMap<String,BlackBoxReportStream>,
    enabled: Option<HashSet<String>>,
    dataset: HashSet<String>,
    stack: Vec<String>,
    stack_name: Option<String>,
    ms_offset: Option<f64>
}

impl BlackBoxStateImpl {
    fn new() -> BlackBoxStateImpl {
        BlackBoxStateImpl {
            pending: HashMap::new(),
            enabled: None,
            dataset: HashSet::new(),
            stack: Vec::new(),
            stack_name: None,
            ms_offset: None
        }
    }
    
    fn get_stream(&mut self, name: &str) -> &mut BlackBoxReportStream {
        self.pending.entry(name.to_string()).or_insert_with(|| {
            BlackBoxReportStream::new(name)
        })
    }
    
    fn set_origin(&mut self, t: f64) {
        if self.ms_offset.is_none() {
            let now : f64 = js! { return +new Date(); }.try_into().unwrap();
            self.ms_offset = Some(now-t);
        }
    }
      
    fn is_enabled(&mut self, stream: &str) -> bool {
        (
            self.enabled.is_none() ||
            self.enabled.as_ref().unwrap().contains(stream) ||
            stream == ""
        )
    }
        
    fn report(&mut self, stream: &str, t: f64, reports: &str) {
        self.set_origin(t);
        if self.is_enabled(stream) {
            let offset = self.ms_offset.unwrap();
            let report = BlackBoxReport::new(
                reports.to_string(),
                self.get_stack_name().to_string(),
                offset+t
            );
            self.get_stream(stream).add_report(report);
        }
    }
    
    fn add_elapsed(&mut self, stream: &str, elapsed: f64) {
        self.get_stream(stream).add_elapsed(elapsed);
    }
    
    fn add_metronome(&mut self, stream: &str, t: f64) {
        self.get_stream(stream).add_metronome(t);
    }
    
    fn make_report(&mut self) -> SerdeValue {
        let mut streams = SerdeMap::<String,SerdeValue>::new();
        if self.enabled.is_some() {
            for (name,reports) in &mut self.pending {
                if self.enabled.as_ref().unwrap().contains(name) {
                    let with_dataset = self.dataset.contains(name);
                    streams.insert(name.to_string(),reports.make_report(with_dataset));
                }
                reports.reset();
            }
        }
        json!({
            "streams": streams,
            "instance_id": get_instance_id()
        })
    }
    
    pub fn set_enabled(&mut self, streams: HashSet<String>) {
        self.enabled = Some(streams.iter().cloned().collect());
    }

    pub fn set_dataset(&mut self, streams: HashSet<String>) {
        self.dataset = streams.iter().cloned().collect();
    }

    fn get_stack_name(&mut self) -> &str {
        if self.stack_name.is_none() {
            self.stack_name = Some(self.stack.join("/"));
        }
        self.stack_name.as_ref().unwrap()
    }

    fn push(&mut self, name: &str) {
        self.stack.push(name.to_string());
        self.stack_name = None;
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.stack_name = None;
    }
}

#[derive(Clone)]
pub struct BlackBoxState(Arc<Mutex<BlackBoxStateImpl>>);

impl BlackBoxState {
    pub(in super) fn new() -> BlackBoxState {
        BlackBoxState(Arc::new(Mutex::new(BlackBoxStateImpl::new())))
    }
    
    pub fn make_report(&mut self) -> SerdeValue {
        self.0.lock().unwrap().make_report()
    }
    
    pub fn is_enabled(&mut self, stream: &str) -> bool {
        self.0.lock().unwrap().is_enabled(stream)
    }
    
    pub fn report(&mut self, stream: &str, t: f64, report: &str) {
        self.0.lock().unwrap().report(stream,t,report);
    }
    
    pub fn elapsed(&mut self, stream: &str, elapsed: f64) {
        self.0.lock().unwrap().add_elapsed(stream,elapsed);
    }
    
    pub fn metronome(&mut self, stream: &str, t: f64) {
        self.0.lock().unwrap().add_metronome(stream,t);
    }
    
    pub fn set_enabled(&mut self, streams: HashSet<String>) {
        self.0.lock().unwrap().set_enabled(streams);
    }

    pub fn set_dataset(&mut self, streams: HashSet<String>) {
        self.0.lock().unwrap().set_dataset(streams);
    }
    
    pub fn push(&mut self, name: &str) {
        self.0.lock().unwrap().push(name);
    }

    pub fn pop(&mut self) {
        self.0.lock().unwrap().pop();
    }    
}
