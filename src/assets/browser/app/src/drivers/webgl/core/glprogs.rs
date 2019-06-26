use std::collections::HashMap;

use dom::webgl::WebGLRenderingContext as glctx;

use drivers::webgl::GLProgData;

use super::super::program::{ Program, GPUSpec, ProgramType };

pub struct GLProgs {
    pub order: Vec<ProgramType>,
    pub map: HashMap<ProgramType,Program>,
}

impl GLProgs {
    pub fn new(ctx: &glctx) -> GLProgs {
        let mut gpuspec = GPUSpec::new();
        gpuspec.populate(&ctx);
        let order = ProgramType::all();
        let mut map = HashMap::<ProgramType,Program>::new();
        for pt in &order {
            bb_log!("webgl-programs","=== {:?} ===",&pt);
            map.insert(*pt,pt.to_program(&gpuspec,&ctx));
        }
        GLProgs { order, map }
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        for p in self.map.values() {
            size += p.size();
        }
        size
    }
    
    pub fn clear_objects(&mut self, ctx: &glctx) {
        for k in &self.order {
            let geom = self.map.get_mut(k).unwrap();
            geom.clear(ctx);
        }        
    }

    pub fn finalize_objects(&mut self, ctx: &glctx, e: &mut GLProgData) {
        for k in &self.order {
            let geom = self.map.get_mut(k).unwrap();
            geom.data.objects_final(ctx,e);
        }
    }
    
    pub fn clean_instance(&self) -> GLProgs {
        GLProgs {
            order: self.order.clone(),
            map: self.map.iter().map(|(k,v)| (*k,v.clean_instance())).collect()
        }
    }
}
