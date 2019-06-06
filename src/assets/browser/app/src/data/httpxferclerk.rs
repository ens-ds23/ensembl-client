use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{ Arc, Mutex };

use itertools::Itertools;
use serde_json::Value as SerdeValue;
use stdweb::unstable::TryInto;
use stdweb::web::{ ArrayBuffer, TypedArray, XmlHttpRequest, XhrResponseType };
use tánaiste::Value;
use url::Url;

use super::{ 
    XferClerk, XferConsumer, XferRequest, XferCache,
    HttpResponseConsumer, HttpManager, BackendConfig
};

use super::backendconfig::BackendBytecode;

struct XferPaceManagerImpl {
    limit: i32,
    underway: i32
}

impl XferPaceManagerImpl {
    pub fn new(limit: i32) -> XferPaceManagerImpl {
        XferPaceManagerImpl {
            limit, underway: 0
        }
    }
    
    pub fn preflight(&mut self) -> bool {
        if self.underway < self.limit {
            self.underway += 1;
            true
        } else {
            false
        }
    }
    
    pub fn land(&mut self) {
        self.underway -= 1;
    }
}

#[derive(Clone)]
struct XferPaceManager(Arc<Mutex<XferPaceManagerImpl>>);

impl XferPaceManager {
    pub fn new(limit: i32) -> XferPaceManager {
        XferPaceManager(Arc::new(Mutex::new(XferPaceManagerImpl::new(limit))))
    }
    
    pub fn preflight(&self) -> bool {
        self.0.lock().unwrap().preflight()
    }
    
    pub fn land(&mut self) {
        self.0.lock().unwrap().land();
    }
}

struct PendingXferRequest {
    consumer: Box<XferConsumer>
}

impl PendingXferRequest {
    fn go(&mut self, bytecode: Rc<BackendBytecode>, data: Vec<Value>) {
        self.consumer.consume(bytecode,data);
    }
}

struct PendingXferBatch {
    config: BackendConfig,
    requests: HashMap<(String,String,String),Vec<PendingXferRequest>>,
    pace: XferPaceManager,
    base: Url,
    cache: XferCache
}

impl PendingXferBatch {
    pub fn new(config: &BackendConfig, base: &Url, pace: &XferPaceManager, cache: &XferCache) -> PendingXferBatch {
        PendingXferBatch {
            config: config.clone(),
            requests: HashMap::<(String,String,String),Vec<PendingXferRequest>>::new(),
            pace: pace.clone(),
            base: base.clone(),
            cache: cache.clone()
        }
    }
    
    pub fn add_request(&mut self, short_stick: &str, short_pane: &str,
                       compo: &str, consumer: Box<XferConsumer>) {
        let key = (short_stick.to_string(),short_pane.to_string(),compo.to_string());
        self.requests.entry(key).or_insert_with(|| {
            Vec::<PendingXferRequest>::new()
        }).push(PendingXferRequest {
            consumer
        });
    }

    pub fn empty(&self) -> bool { self.requests.len() == 0 }

    pub fn fire(self, http_manager: &mut HttpManager) {
        let mut url = self.base.clone();
        let mut url_builder = XferUrlBuilder::new();
        for (short_stick,short_pane,compo) in self.requests.keys() {
            let part = format!("{}^{}/{}",short_stick,short_pane,compo);
            url_builder.add(compo,short_stick,short_pane);
        }
        //console!("url: {:?}",url_builder.emit());
        {
            let mut path = url.path_segments_mut().unwrap();
            path.push(&url_builder.emit());
        }
        let xhr = XmlHttpRequest::new();
        xhr.set_response_type(XhrResponseType::ArrayBuffer);
        xhr.open("GET",&url.as_str());
        http_manager.add_request(xhr,None,Box::new(self));
    }

    fn marshal(&mut self, data: &SerdeValue) -> Vec<Value> {
        let mut out = Vec::<Value>::new();
        for val in unwrap!(data.as_array()) {
            let mut row = Vec::<f64>::new();
            if val.is_array() {
                for cell in unwrap!(val.as_array()) {
                    if cell.is_f64() {
                        row.push(cell.as_f64().unwrap());
                    } else if cell.is_i64() {
                        row.push(cell.as_i64().unwrap() as f64);
                    } else if cell.is_boolean() {
                        row.push(if cell.as_bool().unwrap() { 1. } else { 0. } );
                    }
                }
                out.push(Value::new_from_float(row));
            } else if val.is_string() {
                out.push(Value::new_from_string(val.as_str().unwrap().to_string()));
            }            
        }
        out
    }
}

impl HttpResponseConsumer for PendingXferBatch {
    fn consume(&mut self, req: XmlHttpRequest) {
        let value : ArrayBuffer = ok!(req.raw_response().try_into());
        let value : TypedArray<u8> = value.into();
        let data = ok!(String::from_utf8(value.to_vec()));
        let data : SerdeValue = ok!(serde_json::from_str(&data));
        for resp in data.as_array().unwrap() {
            let key = (resp[0].as_str().unwrap().to_string(),
                       resp[1].as_str().unwrap().to_string(),
                       resp[2].as_str().unwrap().to_string());
            if let Some(mut requests) = self.requests.remove(&key) {
                let codename = resp[3].as_str().unwrap().to_string();
                let bytecode = ok!(self.config.get_bytecode(&codename)).clone();
                let mut recv = (codename,self.marshal(&resp[4]));
                self.cache.put(&key.2,&key.0,&key.1,recv.clone());
                for mut req in requests.drain(..) {
                    req.go(bytecode.clone(),recv.1.clone());
                }
            }
        }
        self.pace.land();
    }
}

struct XferBatchScheduler {
    config: BackendConfig,
    http_manager: HttpManager,
    cache: XferCache,
    url: Url,
    batch: Option<PendingXferBatch>,
    prime_batch: Option<PendingXferBatch>,
    pace: XferPaceManager    
}

impl XferBatchScheduler {
    pub fn new(config: &BackendConfig, http_manager: &HttpManager, cache: &XferCache, 
               url: &Url, pace: i32) -> XferBatchScheduler {
        XferBatchScheduler {
            config: config.clone(),
            http_manager: http_manager.clone(),
            cache: cache.clone(),
            url: url.clone(),
            batch: None,
            prime_batch: None,
            pace: XferPaceManager::new(pace),
        }
    }
    
    fn set_batch(&mut self) {
        self.batch = Some(PendingXferBatch::new(&self.config,&self.url,&self.pace,&self.cache));
    }
    
    pub fn tick(&mut self) -> bool {
        if !self.batch.as_ref().unwrap().empty() && self.pace.preflight() {
            let batch = self.batch.take().unwrap();
            batch.fire(&mut self.http_manager);
            self.set_batch();
            true
        } else {
            false
        }
    }
    
    pub fn add_request(&mut self, short_stick: &str, short_pane: &str,
                       compo: &str, consumer: Box<XferConsumer>) {
        if let Some(ref mut batch) = self.batch {
            batch.add_request(short_stick,short_pane,compo,consumer);
        }
    }
}

pub struct XferUrlBuilder {
    data: HashMap<String,Vec<(String,String)>>
}

impl XferUrlBuilder {
    pub fn new() -> XferUrlBuilder {
        XferUrlBuilder {
            data: HashMap::<String,Vec<(String,String)>>::new()
        }
    }
    
    pub fn add(&mut self, wire: &str, chrom: &str, leaf: &str) {
        let set = self.data.entry(chrom.to_string()).or_insert_with(||
            Vec::<(String,String)>::new()
        );
        set.push((wire.to_string(),leaf.to_string()));
    }
    
    fn emit_chrom(&self, values: &Vec<(String,String)>) -> String {
        let mut data = values.clone();
        data.sort();
        data.iter().map(|(wire,leaf)| format!("{}{}",wire,leaf)).join("")
    }
    
    pub fn emit(&self) -> String {
        let mut chroms = Vec::<(String,String)>::new();
        for (chrom,v) in &self.data {
            chroms.push((chrom.to_string(),self.emit_chrom(v)));
        }
        chroms.sort();
        let chroms : Vec<String> = chroms
                .iter()
                .map(|(chrom,value)| format!("{}:{}",chrom,value))
                .collect();
        chroms.iter().join(",")
    }
}

pub struct HttpXferClerkImpl {
    http_manager: HttpManager,
    config: Option<BackendConfig>,
    base: Url,
    paused: Vec<(XferRequest,Box<XferConsumer>)>,
    batch: Option<XferBatchScheduler>,
    prime_batch: Option<XferBatchScheduler>,
    cache: XferCache
}

impl HttpXferClerkImpl {
    pub fn new(http_manager: &HttpManager, base: &Url, xfercache: &XferCache) -> HttpXferClerkImpl {
        HttpXferClerkImpl {
            http_manager: http_manager.clone(),
            config: None,
            base: base.clone(),
            paused: Vec::<(XferRequest,Box<XferConsumer>)>::new(),
            batch: None,
            prime_batch: None,
            cache: xfercache.clone()
        }
    }

    pub fn tick(&mut self) -> bool {
        let mut any = false;
        if let Some(ref mut batch) = self.batch {
            any |= batch.tick();
        }
        if let Some(ref mut batch) = self.prime_batch {
            any |= batch.tick();
        }
        any
    }

    pub fn set_config(&mut self, bc: BackendConfig) {
        let url = self.base.join(bc.get_data_url()).ok().unwrap();        
        self.config = Some(bc.clone());
        self.batch = Some(XferBatchScheduler::new(&bc,&self.http_manager,&self.cache,&url,5));
        self.prime_batch = Some(XferBatchScheduler::new(&bc,&self.http_manager,&self.cache,&url,1));
        self.batch.as_mut().unwrap().set_batch();
        self.prime_batch.as_mut().unwrap().set_batch();
        /* run requests accumulated during startup */
        let paused : Vec<(XferRequest,Box<XferConsumer>)> = self.paused.drain(..).collect();
        for (request,consumer) in paused {
            self.run_request(request,consumer,false);
        }
    }
    
    pub fn run_request(&mut self, request: XferRequest, mut consumer: Box<XferConsumer>, prime: bool) {
        let leaf = request.get_leaf().clone();
        let (wire,compo) = {
            let compo = request.get_source_name();
            let leaf = request.get_leaf().clone();
            let cfg =  self.config.as_ref().unwrap().clone();
            let track = cfg.get_track(compo).clone();
            (track.and_then(|x| x.get_wire().clone()),compo.clone())
        };
        if let Some(wire) = wire {
            let (short_stick,short_pane) = leaf.get_short_spec();
            if let Some(recv) = self.cache.get(&wire,&short_stick,&short_pane) {
                let bytecode = {
                    let cfg = self.config.as_ref().unwrap().clone();
                    ok!(cfg.get_bytecode(&recv.0)).clone()
                };
                consumer.consume(bytecode,recv.1);
            } else {
                let mut batch = if prime { &mut self.prime_batch } else { &mut self.batch };
                if let Some(ref mut batch) = batch {
                    batch.add_request(&short_stick,&short_pane,&wire,consumer);
                }
            }
        } else {
            consumer.consume(Rc::new(BackendBytecode::noop()),vec!{});
        }
    }
    
    pub fn get_base(&self) -> &Url { &self.base }
}

impl XferClerk for HttpXferClerkImpl {
    fn satisfy(&mut self, request: XferRequest, consumer: Box<XferConsumer>) {
        if self.batch.is_some() {
            let prime = request.get_prime();
            self.run_request(request,consumer,prime);
        } else {
            self.paused.push((request,consumer));
        }
    }
}

#[derive(Clone)]
pub struct HttpXferClerk(Rc<RefCell<HttpXferClerkImpl>>);

impl HttpXferClerk {
    pub fn new(http_manager: &HttpManager, config: &BackendConfig, base: &Url, xfercache: &XferCache) -> HttpXferClerk {
        let mut out = HttpXferClerk(Rc::new(RefCell::new(
            HttpXferClerkImpl::new(http_manager,&base,xfercache))));
        out.set_config(config.clone());
        out
    }

    pub fn set_config(&mut self, bc: BackendConfig) {
        self.0.borrow_mut().set_config(bc);
    }
    
    pub fn tick(&mut self) -> bool {
        self.0.borrow_mut().tick()
    }
}

impl XferClerk for HttpXferClerk {
    fn satisfy(&mut self, request: XferRequest, consumer: Box<XferConsumer>) {
        self.0.borrow_mut().satisfy(request,consumer);
    }
}

