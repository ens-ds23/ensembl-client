use std::collections::HashMap;

use stdweb::web::TypedArray;

use dom::webgl::{
    WebGLRenderingContext as glctx,
    WebGLBuffer as glbuf,
};

use program::data::{ DataBatch };
use program::objects::Object;
use drivers::webgl::GLProgData;

pub struct ObjectMain {
    method: u32,
    vec: HashMap<u32,Vec<u16>>,
    buf: HashMap<u32,glbuf>,
    num: HashMap<u32,u16>,
}

impl ObjectMain {
    pub fn new(method: u32) -> ObjectMain {
        ObjectMain {
            method,
            vec: HashMap::<u32,Vec<u16>>::new(),
            buf: HashMap::<u32,glbuf>::new(),
            num: HashMap::<u32,u16>::new()
        }
    }

    fn buffer(&self, b: &DataBatch) -> Option<&glbuf> {
        self.buf.get(&b.id())
    }

    fn data(&self, b: &DataBatch) -> Option<&Vec<u16>> {
        self.vec.get(&b.id())
    }

    fn data_mut(&mut self, b: &DataBatch) -> &mut Vec<u16> {
        self.vec.entry(b.id()).or_insert_with(|| Vec::<u16>::new())
    }
    
    fn nudge(&mut self, b: &DataBatch, points: u16) -> u16 {
        let bid = b.id();
        let v = *self.num.entry(bid).or_insert(0);
        self.num.insert(bid,v+points);
        v
    }    
}

impl Object for ObjectMain {
    fn is_main(&self) -> bool { true }

    fn add_index(&mut self, batch: &DataBatch, indexes: &[u16], points: u16) {
        let offset = self.nudge(batch,points);
        let b = self.data_mut(batch);
        for v in indexes {
            b.push(v+offset);
        }
    }

    fn obj_final(&mut self, batch: &DataBatch, ctx: &glctx, _acm: &mut GLProgData) {
        self.buf.entry(batch.id()).or_insert_with(|| ctx.create_buffer().unwrap());
        if let Some(data) = self.data(batch) {
            if let Some(buf) = self.buffer(batch) {
                ctx.bind_buffer(glctx::ELEMENT_ARRAY_BUFFER,Some(&buf));
                let data = TypedArray::<u16>::from(&(data[..])).buffer();
                ctx.buffer_data_1(glctx::ELEMENT_ARRAY_BUFFER,Some(&data),glctx::STATIC_DRAW);
            }
        }
    }

    fn execute(&mut self, ctx: &glctx, batch: &DataBatch) {
        if let Some(data) = self.data(batch) {
            if let Some(buf) = self.buffer(batch) {
                ctx.bind_buffer(glctx::ELEMENT_ARRAY_BUFFER,Some(&buf));
                ctx.draw_elements(self.method,data.len() as i32,
                                    glctx::UNSIGNED_SHORT,0);
            }
        }
    }

    fn clear(&mut self, ctx: &glctx) {
        for b in self.buf.values() {
            ctx.delete_buffer(Some(&b));
        }
        self.vec.clear();
        self.buf.clear();
        self.num.clear();
    }
}
