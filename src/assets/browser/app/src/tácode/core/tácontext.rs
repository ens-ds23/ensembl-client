use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use composit::{ ActiveSource, Leaf };
use data::BackendConfig;
use model::shape::DrawingSpec;
use composit::source::SourceResponse;

pub enum TáTask {
    MakeShapes(ActiveSource,Leaf,SourceResponse,Vec<DrawingSpec>,usize,Option<String>,Rc<BackendConfig>)
}

impl TáTask {
    pub fn finished(&mut self) {
        #[allow(unreachable_patterns)]
        match self {
            TáTask::MakeShapes(_,_leaf,sr,_,_,_,_) => {
                //console!("{:?} for {} added {} shapes",leaf,sr.get_source_name(),sr.size());
                sr.done();
            },
            _ => ()
        }
    }
}

pub struct TáContextImpl {
    pid: usize,
    task: Option<TáTask>
}

impl TáContextImpl {
    pub fn new(pid: usize) -> TáContextImpl {
        TáContextImpl { pid, task: None }
    }

    fn set_task(&mut self, task: TáTask) {
        self.task = Some(task);
    }

    fn with_task<F,G>(&mut self, cb: F) -> Option<G>
        where F: FnOnce(&mut TáTask) -> G {
        if let Some(ref mut task) = self.task {
            Some(cb(task))
        } else {
            None
        }
    }

    fn finished(&mut self) {
        if let Some(ref mut task) = self.task {
            task.finished();
        }
    }

    fn appget(&mut self) {
        console!("appget {}",self.pid);
    }
}

#[derive(Clone)]
pub struct TáContext(Rc<RefCell<HashMap<usize,TáContextImpl>>>);

impl TáContext {
    pub fn new() -> TáContext {
        TáContext(Rc::new(RefCell::new(HashMap::<usize,TáContextImpl>::new())))
    }
    
    pub fn set_task(&self, pid: usize, task: TáTask) {
        self.0.borrow_mut().entry(pid).or_insert_with(||
            TáContextImpl::new(pid)
        ).set_task(task);
    } 

    pub fn with_task<F,G>(&self, pid: usize, cb: F) -> Option<G>
            where F: FnOnce(&mut TáTask) -> G {
        self.0.borrow_mut().entry(pid).or_insert_with(||
            TáContextImpl::new(pid)
        ).with_task(cb)
    }    
       
    pub fn appget(&self, pid: usize)  {
        self.0.borrow_mut().entry(pid).or_insert_with(||
            TáContextImpl::new(pid)
        ).appget();
    }
    
    pub fn finished(&self, pid: usize) {
        self.0.borrow_mut().entry(pid).or_insert_with(||
            TáContextImpl::new(pid)
        ).finished();
        self.0.borrow_mut().remove(&pid);
    }
}
