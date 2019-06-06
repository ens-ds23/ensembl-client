use std::cell::RefCell;
use std::rc::Rc;

use stdweb::web::{ XmlHttpRequest, XhrReadyState };

pub trait HttpResponseConsumer {
    fn consume(&mut self,req: XmlHttpRequest);
}

pub struct HttpManagerImpl {
    requests: Vec<(XmlHttpRequest,Box<HttpResponseConsumer>)>
}

impl HttpManagerImpl {
    pub fn new() -> HttpManagerImpl {
        HttpManagerImpl {
            requests: Vec::<(XmlHttpRequest,Box<HttpResponseConsumer>)>::new()
        }
    }

    // TODO error handling
    pub fn add_request(&mut self, req: XmlHttpRequest, data: Option<&[u8]>,
                       consumer: Box<HttpResponseConsumer>) {
        if let Some(data) = data {
            req.send_with_bytes(data);
        } else {
            req.send();
        }
        self.requests.push((req,consumer));
    }
    
    pub fn get_done(&mut self) -> Vec<(XmlHttpRequest,Box<HttpResponseConsumer>)> {
        self.requests.drain_filter(|x|
            x.0.ready_state() == XhrReadyState::Done
        ).collect()
    }
}

#[derive(Clone)]
pub struct HttpManager(Rc<RefCell<HttpManagerImpl>>);

impl HttpManager {
    pub fn new() -> HttpManager {
        HttpManager(Rc::new(RefCell::new(HttpManagerImpl::new())))
    }
    
    pub fn add_request(&self, req: XmlHttpRequest, data: Option<&[u8]>,
                       consumer: Box<HttpResponseConsumer>) {
        self.0.borrow_mut().add_request(req,data,consumer);
    }
    
    pub fn tick(&self) -> bool {
        let done = self.0.borrow_mut().get_done();
        let len = done.len();
        for (req,mut cons) in done {
            cons.consume(req);
        }
        return len != 0
    }
}
