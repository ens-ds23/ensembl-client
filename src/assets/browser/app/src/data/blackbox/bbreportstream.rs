use serde_json::Value as SerdeValue;
use serde_json::Map as SerdeMap;
use serde_json::Number as SerdeNumber;
use stdweb::unstable::TryInto;

use dom::domutil::browser_time;

pub struct BlackBoxReport {
    text: String,
    time: f64,
    stack: String,
    dataset: Option<Vec<f64>>
}

impl BlackBoxReport {
    pub fn new(text: String, stack: String, time: f64) -> BlackBoxReport {
        BlackBoxReport {
            text, time, stack, dataset: None
        }
    }

    pub fn add_dataset(&mut self, data: &Vec<f64>) {
        self.dataset = Some(data.clone());
    }
    
    fn to_json(&self) -> SerdeValue {
        let mut out = json!({
            "time": self.time,
            "stack": self.stack,
            "text": self.text
        });
        if self.dataset.is_some() {
            let dataset_serde = self.dataset.as_ref().unwrap().iter().map(|x| {
                SerdeValue::Number(SerdeNumber::from_f64(*x).unwrap())
            }).collect();
            let dataset_serde = SerdeValue::Array(dataset_serde);
            out.as_object_mut().unwrap().insert("dataset".to_string(),dataset_serde);
        }
        out
    }
}

pub struct BlackBoxReportStream {
    name: String,
    reports: Vec<BlackBoxReport>,
    elapsed: Vec<f64>,
    metronome: Option<f64>
}

impl BlackBoxReportStream {
    pub fn new(name: &str) -> BlackBoxReportStream {
        BlackBoxReportStream {
            name: name.to_string(),
            reports: Vec::new(),
            elapsed: Vec::new(),
            metronome: None
        }
    }
    
    fn analyse_elapsed(&self) -> (usize,f64,f64,f64,f64) {
        let tot = 0.;
        let mut sorted = self.elapsed.clone();
        let num = sorted.len();
        if num > 0 {
            sorted.sort_by(|a,b| a.partial_cmp(b).unwrap());
            let total = sorted.iter().sum();
            let top = sorted.len()-1;
            let high = ((top+1)*19/20).max(1)-1;
            (num,total,total/num as f64,sorted[high],sorted[top])
        } else {
            (0,0.,-1.,-1.,-1.)
        }
    }
    
    fn make_elapsed_report(&mut self, now: f64, with_dataset: bool) {
        let (num,tot,avg,high,top) = self.analyse_elapsed();
        if num > 0 {
            let mut report = BlackBoxReport::new(
                format!("elapsed: num={} total={:.2}ms avg={:.2}ms 95%ile={:.2}ms top={:.2}ms",
                        num,tot,avg,high,top),
                "".to_string(),now
            );
            if with_dataset {
                report.add_dataset(&self.elapsed);
            }
            self.add_report(report);
        }
    }
    
    pub fn reset(&mut self) {
        self.reports.clear();
        self.elapsed.clear();
    }
    
    pub fn make_report(&mut self, with_dataset: bool) -> SerdeValue {
        let now = browser_time();
        self.make_elapsed_report(now,with_dataset);
        let mut stream = SerdeMap::<String,SerdeValue>::new();
        let mut reports = Vec::<SerdeValue>::new();
        for r in &self.reports {
            reports.push(r.to_json());
        }
        stream.insert("reports".to_string(),SerdeValue::Array(reports));
        SerdeValue::Object(stream)
    }
    
    pub fn add_report(&mut self, r: BlackBoxReport) {
        self.reports.push(r);
    }
    
    pub fn add_elapsed(&mut self, elapsed: f64) {
        self.elapsed.push(elapsed);
    }
    
    pub fn add_metronome(&mut self, t: f64) {
        if self.metronome.is_some() {
            let prev = self.metronome.unwrap();
            self.add_elapsed(t-prev);
        }
        self.metronome = Some(t);
    }
}
