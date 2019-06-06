use std::collections::HashMap;
use std::sync::{ Arc, Mutex };

use controller::global::{ AppRunner };
use controller::output::OutputAction;

use serde_json::Map as JSONMap;
use serde_json::Value as JSONValue;
use serde_json::Number as JSONNumber;

#[derive(Clone)]
#[allow(unused)]
pub enum StatusJigsawType {
    Number,
    String,
    Boolean
}

#[derive(Clone)]
#[allow(unused)]
pub enum StatusJigsaw {
    Atom(String,StatusJigsawType),
    Array(Vec<StatusJigsaw>),
    Object(HashMap<String,StatusJigsaw>)
}

struct StatusOutput {
    last_value: Option<JSONValue>,
    jigsaw: StatusJigsaw,
    last_sent: Option<f64>,
    send_every: Option<f64>    
}

impl StatusOutput {
    fn new(jigsaw: StatusJigsaw) -> StatusOutput {
        StatusOutput {
            jigsaw,
            last_sent: None,
            send_every: Some(0.),
            last_value: None,
        }
    }

    fn is_send_now(&self, t: f64) -> bool {
        if let Some(interval) = self.send_every {
            if interval == 0. { return true; }
            if let Some(last_sent) = self.last_sent {
                if t - last_sent > interval { return true; }
            } else {
                return true;
            }
        }
        return false;
    }
}

lazy_static! {
    static ref REPORT_CONFIG:
        Vec<(&'static str,StatusJigsaw,Option<f64>)> = vec!{
        ("location",StatusJigsaw::Array(vec!{
            StatusJigsaw::Atom("stick".to_string(),StatusJigsawType::String),
            StatusJigsaw::Atom("start".to_string(),StatusJigsawType::Number),
            StatusJigsaw::Atom("end".to_string(),StatusJigsawType::Number),
        }),Some(500.)),
        ("bumper",StatusJigsaw::Array(vec!{
            StatusJigsaw::Atom("bumper-top".to_string(),StatusJigsawType::Boolean),
            StatusJigsaw::Atom("bumper-bottom".to_string(),StatusJigsawType::Boolean),
            StatusJigsaw::Atom("bumper-out".to_string(),StatusJigsawType::Boolean),
            StatusJigsaw::Atom("bumper-in".to_string(),StatusJigsawType::Boolean),
            StatusJigsaw::Atom("bumper-left".to_string(),StatusJigsawType::Boolean),
            StatusJigsaw::Atom("bumper-right".to_string(),StatusJigsawType::Boolean),
        }),Some(500.))
    };
}


pub struct ReportImpl {
    pieces: HashMap<String,String>,
    outputs: HashMap<String,StatusOutput>
}

impl ReportImpl {
    pub fn new() -> ReportImpl {
        let out = ReportImpl {
            pieces: HashMap::<String,String>::new(),
            outputs: HashMap::<String,StatusOutput>::new()
        };
        out
    }
    
    fn get_piece(&mut self, key: &str) -> Option<String> {
        self.pieces.get(key).map(|s| s.clone())
    }

    fn get_output(&mut self, key: &str) -> Option<&mut StatusOutput> {
        self.outputs.get_mut(key)
    }
    
    fn add_output(&mut self, key: &str, jigsaw: StatusJigsaw) {
        self.outputs.insert(key.to_string(),StatusOutput::new(jigsaw));
    }
    
    pub fn set_status(&mut self, key: &str, value: &str) {
        self.pieces.insert(key.to_string(),value.to_string());
    }

    pub fn set_interval(&mut self, key: &str, interval: Option<f64>) {
        if let Some(ref mut p) = self.get_output(key) {
            p.send_every = interval;
        }
    }

    fn make_number(&self, data: &str) -> JSONValue {
        data.parse::<f64>().ok()
            .and_then(|v| JSONNumber::from_f64(v) )
            .map(|v| JSONValue::Number(v) )
            .unwrap_or(JSONValue::Null)
    }

    fn make_bool(&self, data: &str) -> JSONValue {
        data.parse::<bool>().ok()
            .map(|v| JSONValue::Bool(v))
            .unwrap_or(JSONValue::Null)
    }

    fn make_atom(&self, key: &str, type_: &StatusJigsawType) -> Option<JSONValue> {
        let v = self.pieces.get(key);
        if let Some(ref v) = v {
            Some(match type_ {
                StatusJigsawType::Number => self.make_number(v),
                StatusJigsawType::String => JSONValue::String(v.to_string()),
                StatusJigsawType::Boolean => self.make_bool(v)
            })
        } else {
            None
        }
    }

    fn make_array(&self, values: &Vec<StatusJigsaw>) -> Option<JSONValue> {
        let mut out = Vec::<JSONValue>::new();
        for v in values {
            if let Some(value) = self.make_value(v) {            
                out.push(value);
            } else {
                return None;
            }
        }
        Some(JSONValue::Array(out))
    }

    fn make_object(&self, values: &HashMap<String,StatusJigsaw>) -> Option<JSONValue> {
        let mut out = JSONMap::<String,JSONValue>::new();
        for (k,v) in values {
            if let Some(value) = self.make_value(v) {
                out.insert(k.to_string(),value);
            } else {
                return None;
            }
        }
        Some(JSONValue::Object(out))
    }

    fn make_value(&self, j: &StatusJigsaw) -> Option<JSONValue> {
        match j {
            StatusJigsaw::Atom(key,type_) => self.make_atom(key,type_),
            StatusJigsaw::Array(values) => self.make_array(values),
            StatusJigsaw::Object(values) => self.make_object(values)
        }
    }

    fn new_report(&mut self, t: f64) -> Option<JSONValue> {
        let mut out = JSONMap::<String,JSONValue>::new();
        for (k,s) in &self.outputs {
            if let Some(value) = self.make_value(&s.jigsaw) {
                if let Some(ref last_value) = s.last_value {
                    if last_value == &value { continue; }
                }
                if s.is_send_now(t) {
                    out.insert(k.to_string(),value.clone());
                }
            }
        }
        for (k,v) in &out {
            if let Some(ref mut p) = self.outputs.get_mut(k) {
                p.last_value = Some(v.clone());
                p.last_sent = Some(t);
            }
        }
        if out.len() > 0 {
            Some(JSONValue::Object(out))
        } else {
            None
        }
    }        
}

#[derive(Clone)]
pub struct Report(Arc<Mutex<ReportImpl>>);

impl Report {
    pub fn new(ar: &mut AppRunner) -> Report {
        let mut out = Report(Arc::new(Mutex::new(ReportImpl::new())));
        for (k,j,v) in REPORT_CONFIG.iter() {
            {
                let mut imp = out.0.lock().unwrap();
                imp.add_output(k,j.clone());
            }
            out.set_interval(k,*v);
        }
        ar.add_timer(enclose! { (out) move |app,t| {
            if let Some(report) = out.new_report(t) {
                vec!{
                    OutputAction::SendCustomEvent("bpane-out".to_string(),report)
                }
            } else { vec!{} }
        }},None);
        out
    }
    
    pub fn set_status(&self, key: &str, value: &str) {
        let mut imp = self.0.lock().unwrap();
        imp.set_status(key,value);
    }    

    pub fn set_status_bool(&self, key: &str, value: bool) {
        self.set_status(key,&value.to_string());
    }    
    
    pub fn set_interval(&mut self, key: &str, interval: Option<f64>) {
        let mut imp = self.0.lock().unwrap();
        imp.set_interval(key,interval);
    }
    
    pub fn new_report(&self, t: f64) -> Option<JSONValue> {
        self.0.lock().unwrap().new_report(t)
    }
}
