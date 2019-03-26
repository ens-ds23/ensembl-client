use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::string::ToString;

use serde_json::Value as SerdeValue;
use tánaiste::Value;
use url::Url;

use composit::{ Leaf, Scale, Stick };

#[derive(Debug,Clone)]
pub struct BackendBytecode {
    pub code: String
}

impl ToString for BackendBytecode {
    fn to_string(&self) -> String {
        self.code.clone()
    }
}

#[derive(Debug,Clone)]
pub struct BackendEndpoint {
    url: Option<String>,
    code: Rc<BackendBytecode>
}

impl BackendEndpoint {
    pub fn get_url(&self) -> Option<&str> { self.url.as_ref().map(|x| &x[..]) }
    pub fn get_code(&self) -> &BackendBytecode { &self.code }
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

/*impl fmt::Debug for BackendAsset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BackendAsset {{ {:?} }}",self)
    }
}*/

#[derive(Debug,Clone)]
pub struct BackendConfig {
    data_url: String,
    assets: HashMap<String,Rc<BackendAsset>>,
    endpoints: HashMap<String,BackendEndpoint>,
    tracks: HashMap<String,BackendTrack>,
    sticks: HashMap<String,Stick>
}

// TODO simplify with serde; error handling
impl BackendConfig {
    pub fn endpoint_for(&self, compo: &str, leaf: &Leaf) -> Result<&BackendEndpoint,String> {
        let scale = leaf.get_scale().get_index();
        let track = self.tracks.get(compo);
        if track.is_none() {
            return Err(format!("No such track {}",compo));
        }
        for (min,max,ep_name) in track.unwrap().endpoints.iter() {
            if scale >= *min && scale <= *max {
                if let Some(ref ep) = self.endpoints.get(ep_name) {
                    return Ok(ep);
                } else {
                    return Err(format!("No such endpoint {}",ep_name));
                }
            }
        }
        return Err(format!("No endpoint for scale {} for {}",scale,compo));
    }
    
    pub fn get_track(&self, name: &str) -> Option<&BackendTrack> {
        self.tracks.get(name)
    }
    
    pub fn get_asset(&self, name: &str) -> Option<&Rc<BackendAsset>> {
        self.assets.get(name)
    }
    
    pub fn get_sticks(&self) -> &HashMap<String,Stick> { &self.sticks }

    pub fn get_data_url(&self) -> &str { &self.data_url }

    fn endpoints_from_json(ep: &SerdeValue, bytecodes: HashMap<String,Rc<BackendBytecode>>) -> HashMap<String,BackendEndpoint> {
        let mut out = HashMap::<String,BackendEndpoint>::new();
        for (k,v) in ep.as_object().unwrap().iter() {
            
            let ep = BackendEndpoint {
                url: v.get("endpoint").map(|x| x.as_str().unwrap().to_string()),
                code: bytecodes[v["bytecode"].as_str().unwrap()].clone()
            };
            out.insert(k.to_string(),ep);
        }
        out
    }
        
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
        for (k,v) in expect!(ep.as_object()).iter() {
            let len : u64 = expectok!(expect!(v.as_str()).to_string().parse());
            out.insert(k.to_string(),Stick::new(k,len,false));
        }
        out
    }

    // TODO errors
    pub fn from_json_string(in_: &str) -> Result<BackendConfig,String> {
        let data : SerdeValue = serde_json::from_str(in_).ok().unwrap();
        let assets = BackendConfig::assets_from_json(&data["assets"],&data["data"]);
        let bytecodes = BackendConfig::bytecodes_from_json(&data["bytecodes"]);
        let endpoints = BackendConfig::endpoints_from_json(&data["endpoints"],bytecodes);
        let tracks = BackendConfig::tracks_from_json(&data["tracks"]);
        let sticks = BackendConfig::sticks_from_json(&data["sticks"]);
        let data_url = data["data-url"].as_str().unwrap().to_string();
        Ok(BackendConfig { assets, endpoints, tracks, sticks, data_url })
    }
}
