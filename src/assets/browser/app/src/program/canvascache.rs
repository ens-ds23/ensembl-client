use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use dom::webgl::WebGLTexture as gltex;

use drivers::webgl::OneCanvasManager;

pub struct CanvasCacheImpl {
    cache: HashMap<(u32,u32),gltex>
}

impl CanvasCacheImpl {
    pub fn new() -> CanvasCacheImpl {
        CanvasCacheImpl {
            cache: HashMap::<(u32,u32),gltex>::new()
        }
    }
    
    pub fn find_texture(&mut self, ocm: &OneCanvasManager) -> Option<gltex> {
        let out = self.cache.get(&ocm.get_full_idx());
        out.cloned()
    }
    
    pub fn set_texture(&mut self, ocm: &OneCanvasManager, gltex: &gltex) {
        self.cache.insert(ocm.get_full_idx(),gltex.clone());
    }
}

#[derive(Clone)]
pub struct CanvasCache(Rc<RefCell<CanvasCacheImpl>>);

impl CanvasCache {
    pub fn new() -> CanvasCache {
        CanvasCache(Rc::new(RefCell::new(CanvasCacheImpl::new())))
    }

    pub fn find_texture(&self, ocm: &OneCanvasManager) -> Option<gltex> {
        self.0.borrow_mut().find_texture(ocm)
    }
    
    pub fn set_texture(&self, ocm: &OneCanvasManager, gltex: &gltex) {
        self.0.borrow_mut().set_texture(ocm,gltex);
    }
}
