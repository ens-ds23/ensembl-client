use std::rc::Rc;
use std::cell::RefCell;

use tánaiste::Value;

use composit::Leaf;
use data::{ BackendConfig, XferClerk, XferConsumer, XferRequest };
use util::Cache;

struct XferPrimeConsumer(String,String,String,XferCache);
impl XferConsumer for XferPrimeConsumer {
    fn consume(&mut self, code: String, data: Vec<Value>) {
        self.3.put(&self.0,&self.1,&self.2,(code,data));
    }
    fn abandon(&mut self) {}
}

pub struct XferCacheImpl {
    cache: Cache<(String,String,String),(String,Vec<Value>)>
}

impl XferCacheImpl {
    pub fn new(size: usize) -> XferCacheImpl {
        XferCacheImpl {
            cache: Cache::new(size)
        }
    }
    
    pub fn put(&mut self, compo: &str, short_stick: &str, short_pane: &str, values: (String,Vec<Value>)) {
        self.cache.put(&(compo.to_string(),short_stick.to_string(),short_pane.to_string()),values);
    }
    
    pub fn get(&mut self, compo: &str, short_stick: &str, short_pane: &str) -> Option<(String,Vec<Value>)> {
        self.cache.get(&(compo.to_string(),short_stick.to_string(),short_pane.to_string())).cloned()
    }    
}

#[derive(Clone)]
pub struct XferCache(Rc<RefCell<XferCacheImpl>>,BackendConfig);

impl XferCache {
    pub fn new(size: usize, config: &BackendConfig) -> XferCache {
        XferCache(Rc::new(RefCell::new(XferCacheImpl::new(size))),config.clone())
    }

    pub fn put(&mut self, compo: &str, short_stick: &str, short_pane: &str, values: (String,Vec<Value>)) {
        //console!("put compo={:?} stick={:?} pane={:?}",compo,short_stick,short_pane);
        self.0.borrow_mut().put(compo,short_stick,short_pane,values);
    }
    
    pub fn get(&mut self, compo: &str, short_stick: &str, short_pane: &str) -> Option<(String,Vec<Value>)> {
        //console!("get compo={:?} stick={:?} pane={:?}",compo,short_stick,short_pane);
        self.0.borrow_mut().get(compo,short_stick,short_pane)
    }
    
    pub fn prime(&mut self, xferclerk: &mut Box<XferClerk>, compo: &str, leaf: &Leaf) {
        let wire = self.1.get_track(compo)
                        .and_then(|x| x.get_wire().as_ref())
                        .map(|x| x.to_string());
        if let Some(wire) = wire {
            let (short_stick,short_pane) = leaf.get_short_spec();
            if self.get(&wire,&short_stick,&short_pane).is_none() {
                xferclerk.satisfy(XferRequest::new(compo,leaf,true),
                    Box::new(XferPrimeConsumer(
                        wire.to_string(),
                        short_stick,
                        short_pane,
                        self.clone()
                    )));
            }
        }
    }
}
