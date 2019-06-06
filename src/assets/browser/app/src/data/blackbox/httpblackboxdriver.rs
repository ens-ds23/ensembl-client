use std::collections::HashSet;
use std::sync::{ Arc, Mutex };

use serde_json::Value as SerdeValue;
use stdweb::unstable::TryInto;
use stdweb::web::{ ArrayBuffer, TypedArray, XmlHttpRequest, XhrResponseType };
use url::Url;

use super::{ BlackBoxDriverImpl, BlackBoxState };
use data::{ HttpManager, HttpResponseConsumer };

const DEFAULT_REPORT_INTERVAL : f64 = 600000.; // ms ie 10min

pub struct BlackBoxResponseConsumer {
    state: BlackBoxState,
    interval: Arc<Mutex<f64>>
}

impl BlackBoxResponseConsumer {
    pub fn new(state: &BlackBoxState, interval: Arc<Mutex<f64>>) -> BlackBoxResponseConsumer {
        BlackBoxResponseConsumer {
            state: state.clone(),
            interval
        }
    }
}

fn serde_to_vec_string(in_: &SerdeValue, key: &str) -> Option<Vec<String>> {
    let mut out = None;
    if let SerdeValue::Object(map) = in_ {
        if let Some(SerdeValue::Array(enabled)) = map.get(key) {
            let mut v : Vec<String> = vec![];
            for stream in enabled {
                if let SerdeValue::String(string) = stream {
                    v.push(string.clone());
                }
            }
            out = Some(v);
        }
    }
    out
}

fn serde_to_set_string(in_: &SerdeValue, key: &str) -> Option<HashSet<String>> {
    serde_to_vec_string(in_,key).and_then(|mut x| {
        Some(x.drain(..).collect())
    })
}

fn serde_to_number(in_: &SerdeValue, key: &str) -> Option<f64> {
    let mut out = None;
    if let SerdeValue::Object(map) = in_ {
        if let Some(SerdeValue::Number(v)) = map.get(key) {
            out = v.as_f64();
        }
    }
    out    
}

impl HttpResponseConsumer for BlackBoxResponseConsumer {
    fn consume(&mut self, req: XmlHttpRequest) {
        let value : ArrayBuffer = ok!(req.raw_response().try_into());
        let value : TypedArray<u8> = value.into();
        let data = ok!(String::from_utf8(value.to_vec()));
        let data : SerdeValue = ok!(serde_json::from_str(&data));
        if let Some(mut enabled) = serde_to_set_string(&data,"enabled") {
            if let Some(dataset) = serde_to_set_string(&data,"dataset") {
                enabled = enabled.union(&dataset).cloned().collect();
            }
            self.state.set_enabled(enabled);            
        }
        if let Some(dataset) = serde_to_set_string(&data,"dataset") {
            self.state.set_dataset(dataset);
        }
        if let Some(interval) = serde_to_number(&data,"interval") {
            *self.interval.lock().unwrap() = interval*1000.;
        }
    }
}

pub struct HttpBlackBoxDriverImpl {
    manager: HttpManager,
    url: Url,
    report_interval: Arc<Mutex<f64>>,
    last_report: Option<f64>
}

impl HttpBlackBoxDriverImpl {
    pub fn new(http_manager: &HttpManager, base: &Url) -> HttpBlackBoxDriverImpl {
        HttpBlackBoxDriverImpl {
            manager: http_manager.clone(),
            report_interval: Arc::new(Mutex::new(DEFAULT_REPORT_INTERVAL)),
            url: base.clone(),
            last_report: None
        }
    }
}

impl BlackBoxDriverImpl for HttpBlackBoxDriverImpl {
    fn tick(&mut self, state: &mut BlackBoxState, t: f64) -> bool {
        let interval : f64 = self.report_interval.lock().unwrap().clone();
        if self.last_report == None || t-self.last_report.unwrap() > interval {
            let report = state.make_report().to_string();
            let xhr = XmlHttpRequest::new();
            xhr.open("POST",&self.url.as_str());
            xhr.set_request_header("Content-Type", "application/json");
            xhr.set_response_type(XhrResponseType::ArrayBuffer);
            let data = report.as_bytes();
            let interval = self.report_interval.clone();
            self.manager.add_request(xhr,Some(data),Box::new(BlackBoxResponseConsumer::new(state,interval)));
            self.last_report = Some(t);
            true
        } else {
            false
        }
    }
}
