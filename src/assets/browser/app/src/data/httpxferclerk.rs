use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{ Arc, Mutex };

use serde_json::Value as SerdeValue;
use stdweb::unstable::TryInto;
use stdweb::web::{ ArrayBuffer, TypedArray, XmlHttpRequest, XhrResponseType };
use tánaiste::Value;
use url::Url;

use composit::Leaf;
use util::Cache;
use super::{ 
    XferClerk, XferConsumer, XferRequest, XferCache,
    HttpResponseConsumer, HttpManager, BackendConfig,
    BackendConfigBootstrap
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
    code: String,
    consumer: Box<XferConsumer>
}

impl PendingXferRequest {
    fn go(&mut self, recv: Vec<Value>) {
        self.consumer.consume(self.code.clone(),recv);
    }
}

struct PendingXferBatch {
    requests: HashMap<(String,String,String),Vec<PendingXferRequest>>,
    pace: XferPaceManager,
    base: Url,
    cache: XferCache
}

impl PendingXferBatch {
    pub fn new(base: &Url, pace: &XferPaceManager, cache: &XferCache) -> PendingXferBatch {
        PendingXferBatch {
            requests: HashMap::<(String,String,String),Vec<PendingXferRequest>>::new(),
            pace: pace.clone(),
            base: base.clone(),
            cache: cache.clone()
        }
    }
    
    pub fn add_request(&mut self, name: &str, leaf_spec: &str,
                       code: &str, compo: &str, consumer: Box<XferConsumer>) {
        let key = (name.to_string(),leaf_spec.to_string(),compo.to_string());
        self.requests.entry(key).or_insert_with(|| {
            Vec::<PendingXferRequest>::new()
        }).push(PendingXferRequest {
            code: code.to_string(),
            consumer
        });
    }    

    pub fn empty(&self) -> bool { self.requests.len() == 0 }

    pub fn fire(self, http_manager: &mut HttpManager) {
        let mut url = self.base.clone();
        for (name,leaf_spec,compo) in self.requests.keys() {
            let mut qp = url.query_pairs_mut();
            let part = format!("{}/{}/{}",name,leaf_spec,compo);
            qp.append_pair("parts",&part);
        }
        let xhr = XmlHttpRequest::new();
        xhr.set_response_type(XhrResponseType::ArrayBuffer);
        xhr.open("GET",&url.as_str());
        http_manager.add_request(xhr,None,Box::new(self));
    }

    fn marshal(&mut self, data: &SerdeValue) -> Vec<Value> {
        let mut out = Vec::<Value>::new();
        for val in data.as_array().unwrap() {
            let mut row = Vec::<f64>::new();
            if val.is_array() {
                for cell in val.as_array().unwrap() {
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
        let value : ArrayBuffer = req.raw_response().try_into().ok().unwrap();
        let value : TypedArray<u8> = value.into();
        let data = String::from_utf8(value.to_vec()).ok().unwrap();
        let data : SerdeValue = serde_json::from_str(&data).ok().unwrap();
        for resp in data.as_array().unwrap() {
            let key = (resp[0].as_str().unwrap().to_string(),
                       resp[1].as_str().unwrap().to_string(),
                       resp[2].as_str().unwrap().to_string());
            if let Some(mut requests) = self.requests.remove(&key) {
                let mut recv = self.marshal(&resp[3]);
                for mut req in requests.drain(..) {
                    self.cache.put(&key.2,&key.1,recv.clone());
                    req.go(recv.clone());
                }
            }
        }
        self.pace.land();
    }
}

struct XferBatchScheduler {
    http_manager: HttpManager,
    cache: XferCache,
    url: Url,
    batch: Option<PendingXferBatch>,
    prime_batch: Option<PendingXferBatch>,
    pace: XferPaceManager    
}

impl XferBatchScheduler {
    pub fn new(http_manager: &HttpManager, cache: &XferCache, 
               url: &Url, pace: i32) -> XferBatchScheduler {
        XferBatchScheduler {
            http_manager: http_manager.clone(),
            cache: cache.clone(),
            url: url.clone(),
            batch: None,
            prime_batch: None,
            pace: XferPaceManager::new(pace),
        }
    }
    
    fn set_batch(&mut self) {
        self.batch = Some(PendingXferBatch::new(&self.url,&self.pace,&self.cache));
    }
    
    pub fn tick(&mut self) {
        if !self.batch.as_ref().unwrap().empty() && self.pace.preflight() {
            let batch = self.batch.take().unwrap();
            batch.fire(&mut self.http_manager);
            self.set_batch();
        }        
    }
    
    pub fn add_request(&mut self, name: &str, leaf_spec: &str,
                       code: &str, compo: &str, consumer: Box<XferConsumer>) {
        if let Some(ref mut batch) = self.batch {
            batch.add_request(name,leaf_spec,code,compo,consumer);
        }
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
        let mut out = HttpXferClerkImpl {
            http_manager: http_manager.clone(),
            config: None,
            base: base.clone(),
            paused: Vec::<(XferRequest,Box<XferConsumer>)>::new(),
            batch: None,
            prime_batch: None,
            cache: xfercache.clone()
        };
        out
    }

    pub fn tick(&mut self) {
        if let Some(ref mut batch) = self.batch {
            batch.tick();
        }
        if let Some(ref mut batch) = self.prime_batch {
            batch.tick();
        }
    }

    pub fn set_config(&mut self, bc: BackendConfig) {
        let mut url = self.base.join(bc.get_data_url()).ok().unwrap();        
        self.config = Some(bc.clone());
        self.batch = Some(XferBatchScheduler::new(&self.http_manager,&self.cache,&url,5));
        self.prime_batch = Some(XferBatchScheduler::new(&self.http_manager,&self.cache,&url,1));
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
        let (name,code,compo) = {
            let compo = request.get_source_name();
            let leaf = request.get_leaf().clone();
            let cfg =  self.config.as_ref().unwrap().clone();
            let endpoint = cfg.endpoint_for(compo,&leaf).clone();
            if endpoint.is_err() {
                //console!("No data for {:?}: {}",
                //            request.get_leaf().clone(),
                //            endpoint.unwrap_err());
                consumer.abandon();
                return;
            }
            let endpoint = endpoint.as_ref().unwrap();
            (endpoint.get_url().map(|x| x.to_string().clone()),endpoint.get_code().clone(),compo)
        };
        if let Some(name) = name {
            if let Some(recv) = self.cache.get(&compo,&leaf.get_spec()) {
                consumer.consume(code.to_string(),recv);
            } else {
                let mut batch = if prime { &mut self.prime_batch } else { &mut self.batch };
                if let Some(ref mut batch) = batch {
                    batch.add_request(&name,&leaf.get_spec(),&code.to_string(),&compo,consumer);
                }
            }
        } else {
            consumer.consume(code.to_string(),vec!{});
        }
    }
    
    pub fn get_base(&self) -> &Url { &self.base }
}

impl XferClerk for HttpXferClerkImpl {
    fn satisfy(&mut self, request: XferRequest, mut consumer: Box<XferConsumer>) {
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
    
    pub fn tick(&mut self) {
        self.0.borrow_mut().tick();
    }
}

impl XferClerk for HttpXferClerk {
    fn satisfy(&mut self, request: XferRequest, mut consumer: Box<XferConsumer>) {
        self.0.borrow_mut().satisfy(request,consumer);
    }
}

