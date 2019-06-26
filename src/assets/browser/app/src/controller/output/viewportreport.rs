use std::sync::{ Arc, Mutex };

use serde_json::Map as JSONMap;
use serde_json::Value as JSONValue;
use serde_json::Number as JSONNumber;


use controller::global::{ App, AppRunner };
use controller::output::OutputAction;
use util::ChangeDetect;

#[derive(Debug)]
enum ViewportReportItem {
    DeltaY(i32),
    TrackY(Vec<(String,i32)>)
}

impl ViewportReportItem {
    pub fn to_json(&self, map: &mut JSONMap<String,JSONValue>) {
        match self {
            ViewportReportItem::DeltaY(dy) => {
                let v = JSONValue::Number(JSONNumber::from_f64(*dy as f64).unwrap());
                map.insert("delta_y".to_string(),v);
            },
            ViewportReportItem::TrackY(yy) => {
                let mut json_y = JSONMap::<String,JSONValue>::new();
                for y in yy {
                    json_y.insert(y.0.clone(),JSONValue::Number(JSONNumber::from_f64(y.1 as f64).unwrap()));
                }
                map.insert("track_y".to_string(),JSONValue::Object(json_y));
            }
        }
    }
}

pub struct ViewportReportImpl {
    delta_y: ChangeDetect<i32>,
    track_y: ChangeDetect<Vec<(String,i32)>>
}

impl ViewportReportImpl {
    pub fn new() -> ViewportReportImpl {
        ViewportReportImpl {
            delta_y: ChangeDetect::<i32>::new(),
            track_y: ChangeDetect::<Vec<(String,i32)>>::new()
        }
    }
    
    fn set_delta_y(&mut self, y: i32) {
        self.delta_y.set(y);
    }
    
    fn get_items(&mut self, app: &mut App) -> Vec<ViewportReportItem> {
        let mut out = Vec::<ViewportReportItem>::new();
        if let Some(dy) = self.delta_y.report() {
            out.push(ViewportReportItem::DeltaY(dy));
        }        
        let mut all_pos = Vec::<(String,i32)>::new();
        app.get_all_landscapes().every(|_lid,ls| {
            let plot = ls.get_plot();
            if plot.has_cog() {
                all_pos.push((ls.get_name().to_string(),plot.get_base()));
            }
        });
        self.track_y.set(all_pos);
        if let Some(dy) = self.track_y.report() {
            out.push(ViewportReportItem::TrackY(dy));
        }        
        out
    }
    
    fn build_value(&self, items: Vec<ViewportReportItem>) -> JSONValue {
        let mut out = JSONMap::<String,JSONValue>::new();
        for item in items {
            item.to_json(&mut out);
        }
        JSONValue::Object(out)
    }
    
    fn make_report(&mut self, app: &mut App) -> Option<JSONValue> {
        let items = self.get_items(app);
        if items.len() > 0 {
            Some(self.build_value(items))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct ViewportReport(Arc<Mutex<ViewportReportImpl>>);

impl ViewportReport {
    pub fn new(ar: &mut AppRunner) -> ViewportReport {
        let out = ViewportReport(Arc::new(Mutex::new(ViewportReportImpl::new())));
        ar.add_timer("viewport-report",enclose! { (out) move |app,_,sr| {
            if let Some(report) = out.clone().make_report(app) {
                vec!{
                    OutputAction::SendCustomEvent("bpane-scroll".to_string(),report)
                }
            } else {
                sr.unproductive();
                vec!{}
            }
        }},4);
        out
    }
    
    pub fn set_delta_y(&self, y: i32) {
        self.0.lock().unwrap().set_delta_y(y);
    }
    
    pub fn make_report(&self, app: &mut App) -> Option<JSONValue> {
        self.0.lock().unwrap().make_report(app)
    }
}
