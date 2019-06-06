use std::collections::HashMap;
use std::rc::Rc;
use std::string::ToString;

use serde_json::Value as SerdeValue;
use tánaiste::Value;

use composit::{ Scale, Stick };

#[derive(Debug,Clone)]
pub struct BackendBytecode {
    pub code: String
}

impl BackendBytecode {
    pub fn noop() -> BackendBytecode {
        BackendBytecode { code: "".to_string() }
    }
    
    pub fn get_source(&self) -> &str { &self.code }
}

#[derive(Debug,Clone)]
pub struct BackendTrack {
    endpoints: Vec<(i32,i32,String)>,
    letter: String,
    wire: Option<String>,
    position: i32,
    parts: Vec<String>
}

impl BackendTrack {
    pub fn get_letter(&self) -> &str { &self.letter }
    pub fn get_position(&self) -> i32 { self.position }
    pub fn get_parts(&self) -> &Vec<String> { &self.parts }
    pub fn get_wire(&self) -> &Option<String> { &self.wire }
}

#[derive(Debug)]
pub struct BackendAsset {
    data: Vec<Value>
}

impl BackendAsset {
    pub fn get_stream(&self, index: usize) -> &Value {
        &self.data[index]
    }
}

// TODO: Protect with Rc
#[derive(Debug,Clone)]
pub struct BackendConfig {
    data_url: String,
    assets: HashMap<String,Rc<BackendAsset>>,
    tracks: HashMap<String,BackendTrack>,
    sticks: HashMap<String,Stick>,
    bytecodes: HashMap<String,Rc<BackendBytecode>>
}

// TODO simplify with serde; error handling
impl BackendConfig {
    pub fn get_bytecode(&self, name: &str) -> Result<&Rc<BackendBytecode>,String> {
        self.bytecodes.get(name).ok_or_else(||
            format!("No bytecode named {}",name))
    }
    
    pub fn get_track(&self, name: &str) -> Option<&BackendTrack> {
        self.tracks.get(name)
    }
    
    pub fn get_asset(&self, name: &str) -> Option<&Rc<BackendAsset>> {
        self.assets.get(name)
    }
    
    pub fn get_sticks(&self) -> &HashMap<String,Stick> { &self.sticks }

    pub fn get_data_url(&self) -> &str { &self.data_url }

    fn tracks_from_json(ep: &SerdeValue) -> HashMap<String,BackendTrack> {
        let mut out = HashMap::<String,BackendTrack>::new();
        for (track_name,v) in ep.as_object().unwrap().iter() {
            let mut endpoints = Vec::<(i32,i32,String)>::new();
            for (scales,track) in v["endpoints"].as_object().unwrap().iter() {
                let scales :Vec<char> = scales.chars().collect();
                let min = Scale::new_from_letter(scales[0]).get_index();
                let max = Scale::new_from_letter(scales[1]).get_index();
                endpoints.push((min,max,track["endpoint"].as_str().unwrap().to_string()));
            }
            let mut parts = Vec::<String>::new();
            for part in v["parts"].as_array().unwrap_or(&vec!{}).iter() {
                parts.push(part.as_str().unwrap().to_string());
            }
            let track_name = format!("track:{}",track_name);
            out.insert(track_name,BackendTrack { 
                letter: v.get("letter").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                position: v.get("position").and_then(|x| x.as_i64()).unwrap_or(-1) as i32,
                wire: v.get("wire").and_then(|x| x.as_str()).map(|x| x.to_string()),
                endpoints, parts
            });
        }
        out
    }

    fn bytecodes_from_json(ep: &SerdeValue) -> HashMap<String,Rc<BackendBytecode>> {
        let mut out = HashMap::<String,Rc<BackendBytecode>>::new();
        for (k,v) in ep.as_object().unwrap().iter() {
            let ep = BackendBytecode {
                code: v.as_str().unwrap().to_string()
            };
            out.insert(k.to_string(),Rc::new(ep));
        }
        out.insert("".to_string(),Rc::new(BackendBytecode::noop()));
        out
    }
    
    fn one_asset_from_json(name: &str, data_in: &SerdeValue) -> Rc<BackendAsset> {
        let mut data = Vec::<Value>::new();
        for v in data_in[name].as_array().unwrap_or(&vec!{}).iter() {
            if v.is_string() {
                data.push(Value::new_from_string(v.as_str().unwrap().to_string()));
            } else {
                let mut array = Vec::<f64>::new();
                for x in v.as_array().unwrap_or(&vec!{}).iter() {
                    array.push(x.as_f64().unwrap());
                }
                data.push(Value::new_from_float(array));
            }
        }
        Rc::new(BackendAsset { data })
    }
    
    fn assets_from_json(assets: &SerdeValue, data: &SerdeValue) -> HashMap<String,Rc<BackendAsset>> {
        let mut out = HashMap::<String,Rc<BackendAsset>>::new();
        for (name,v) in assets.as_object().unwrap().iter() {
            out.insert(name.to_string(),BackendConfig::one_asset_from_json(name,data));
        }
        out
    }
    
    fn sticks_from_json(ep: &SerdeValue) -> HashMap<String,Stick> {
        let mut out = HashMap::<String,Stick>::new();
        for (k,v) in unwrap!(ep.as_object()).iter() {
            let len : u64 = ok!(unwrap!(v.as_str()).to_string().parse());
            out.insert(k.to_string(),Stick::new(k,len,false));
        }
        out
    }

    // TODO errors
    pub fn from_json_string(in_: &str) -> Result<BackendConfig,String> {
        let data : SerdeValue = serde_json::from_str(in_).ok().unwrap();
        let assets = BackendConfig::assets_from_json(&data["assets"],&data["data"]);
        let bytecodes = BackendConfig::bytecodes_from_json(&data["bytecodes"]);
        let tracks = BackendConfig::tracks_from_json(&data["tracks"]);
        let sticks = BackendConfig::sticks_from_json(&data["sticks"]);
        let data_url = data["data-url"].as_str().unwrap().to_string();
        Ok(BackendConfig { assets, tracks, sticks, data_url, bytecodes })
    }
}
