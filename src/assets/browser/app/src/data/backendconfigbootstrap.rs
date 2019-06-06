use std::cell::RefCell;
use std::rc::Rc;

use stdweb::web::{XmlHttpRequest, XhrResponseType };
use url::Url;

use super::{ BackendConfig, HttpManager, HttpResponseConsumer };

pub struct BackendConfigBootstrapImpl {
    config: Option<BackendConfig>,
    callbacks: Vec<Box<Fn(&BackendConfig)>>
}

impl BackendConfigBootstrapImpl {
    fn new() -> BackendConfigBootstrapImpl {
        BackendConfigBootstrapImpl {
            config: None,
            callbacks: Vec::<Box<Fn(&BackendConfig)>>::new()
        }
    }
    
    fn add_callback(&mut self, cb: Box<Fn(&BackendConfig)>) {
        match self.config {
            Some(ref mut cfg) => cb(&cfg),
            None => self.callbacks.push(cb)
        };
    }
}

impl HttpResponseConsumer for BackendConfigBootstrapImpl {
    fn consume(&mut self, req: XmlHttpRequest) {
        let value = req.response_text().ok().unwrap().unwrap();
        let backend = BackendConfig::from_json_string(&value).expect("couldn't get remote config");
        for cb in self.callbacks.drain(..) {
            cb(&backend);
        }
    }
}

#[derive(Clone)]
pub struct BackendConfigBootstrap(Rc<RefCell<BackendConfigBootstrapImpl>>,Url);

impl BackendConfigBootstrap {
    pub fn new(http_manager: &HttpManager, base: &Url) -> BackendConfigBootstrap {
        let imp = BackendConfigBootstrapImpl::new();
        let out = BackendConfigBootstrap(Rc::new(RefCell::new(imp)),base.clone());
        let xhr = XmlHttpRequest::new();
        xhr.set_response_type(XhrResponseType::Text);
        xhr.open("GET",&base.as_str());
        http_manager.add_request(xhr,None,Box::new(out.clone()));
        out
    }
    
    pub fn add_callback(&mut self, cb: Box<Fn(&BackendConfig)>) {
        self.0.borrow_mut().add_callback(cb);
    }
    
    pub fn get_base(&self) -> &Url { &self.1 }
}

impl HttpResponseConsumer for BackendConfigBootstrap {
    fn consume(&mut self, req: XmlHttpRequest) {
        self.0.borrow_mut().consume(req);
    }
}
