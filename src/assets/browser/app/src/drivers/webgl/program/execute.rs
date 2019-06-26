use std::collections::HashMap;
use std::rc::Rc;

use dom::webgl::{
    WebGLRenderingContext as glctx,
    WebGLProgram as glprog,
};

use super::source::{ Source, ProgramSource };
use super::objects::Object;
use super::data::{ DataBatch, DataGroupIndex, BatchManager };
use super::gpuspec::GPUSpec;
use super::impls::ProgramType;
use drivers::webgl::GLProgData;

pub struct ProgramAttribs {
    pt: ProgramType,
    pub bman: BatchManager,
    default_group: DataGroupIndex,
    pub objects: Vec<Box<Object>>,
    main_idx: Option<usize>,
    object_names: HashMap<String,usize>,
    source: Vec<Rc<Source>>,
}

pub struct Program {
    pub data: ProgramAttribs,
    prog: Rc<glprog>,
}

fn make_objects(vars: &Vec<Rc<Source>>, prog: Rc<glprog>) 
                        -> (Vec<Box<Object>>,Option<usize>,
                            HashMap<String,usize>) {
    let mut main = None;
    let mut objects = Vec::<Box<Object>>::new();
    let mut object_names = HashMap::<String,usize>::new();
    for v in vars {
        if let Some((name,value)) = v.create(prog.clone()) {
            if value.is_main() { main = Some(objects.len()); }
            let loc = objects.len();
            objects.push(value);
            if let Some(name) = name {
                object_names.insert(name.to_string(),loc);
            }
        }
    }
    (objects,main,object_names)
}

impl ProgramAttribs {
    pub fn new(pt: ProgramType, src: &Vec<Rc<Source>>, prog: &Rc<glprog>) -> ProgramAttribs {
        let mut bman = BatchManager::new();
        let default_group = bman.new_group();
        let (objects,main_idx,object_names) = make_objects(&src,prog.clone());
        ProgramAttribs {
                bman, default_group, pt,
                objects, object_names, main_idx,
                source: src.clone()
        }
    }
        
    pub fn get_object(&mut self, name: &str) -> Option<&mut Box<Object>> {
        if let Some(idx) = self.object_names.get(name) {
            self.objects.get_mut(*idx)
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        let mut size = 0;
        for obj in &self.objects {
            size += obj.size();
        }
        size
    }

    fn clear(&mut self, ctx: &glctx) {
        for a in &mut self.objects.iter_mut() {
            a.clear(ctx);
        }
        self.default_group = self.bman.new_group();
    }

    pub fn objects_final(&mut self, ctx: &glctx, e: &mut GLProgData) {
        for b in self.bman.batches() {
            for a in &mut self.objects.iter_mut() {
                a.obj_final(&b,ctx,e);
            }
        }
    }

    pub fn prog_type(&self) -> &ProgramType { &self.pt }
    
    pub fn get_default_group(&self) -> DataGroupIndex {
        self.default_group
    }
    
    pub fn new_group(&mut self) -> DataGroupIndex {
        self.bman.new_group()
    }
    
    pub fn add_vertices(&mut self, group: DataGroupIndex,
                        indexes: &[u16], points: u16) -> DataBatch {
        let b = self.bman.get_batch(&group,points);
        if let Some(obj_idx) = self.main_idx {
            let main = &mut self.objects[obj_idx];
            main.add_index(&b,indexes,points);
        }
        b
    }
}

impl Program {
    pub fn new(pt: ProgramType, gpuspec: &GPUSpec, ctx: &glctx, src: &ProgramSource) -> Program {
        let prog = Rc::new(src.prog(gpuspec,ctx));
        Program {
            data: ProgramAttribs::new(pt,&src.uniforms,&prog),
            prog,
        }
    }

    pub fn clean_instance(&self) -> Program {
        Program {
            data: ProgramAttribs::new(self.data.pt,&self.data.source,&self.prog),
            prog: self.prog.clone()
        }
    }

    pub fn size(&self) -> usize {
        self.data.size()
    }

    pub fn clear(&mut self, ctx: &glctx) {
        /* Must be in this order */
        self.data.bman.reset();
        self.data.clear(ctx);
    }

    pub fn use_program(&self, ctx: &glctx) {
        ctx.use_program(Some(&self.prog));
    }
  
    pub fn get_object(&mut self, name: &str) -> Option<&mut Box<Object>> {
        self.data.get_object(name)
    }
  
    pub fn execute(&mut self, ctx: &glctx) {
        self.use_program(ctx);
        for b in self.data.bman.batches() {
            let mut main = None;
            for a in &mut self.data.objects {
                if a.is_main() {
                    main = Some(a);
                } else {
                    a.execute(ctx,&b);
                }
            }
            if let Some(a) = main {
                a.execute(ctx,&b);
            }
        }
    }
}
