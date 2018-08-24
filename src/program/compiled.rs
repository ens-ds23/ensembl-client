use std::collections::HashMap;
use std::rc::Rc;

use webgl_rendering_context::{
    WebGLRenderingContext as glctx,
    WebGLBuffer as glbuf,
    WebGLProgram as glprog,
    WebGLUniformLocation as gluni,
};

use arena::{ Stage, ArenaData, ArenaDims };
use coord::Input;

use shape::ShapeManager;
use texture::TextureTargetManager;

use program::source::{ Source, ProgramSource };
use program::objects::Object;

pub struct ProgramAttribs {
    indices: i32,
    attribs: Vec<Box<Object>>,
    attrib_names: HashMap<String,usize>,
}

pub struct ProgramCode {
    uniforms: HashMap<String,gluni>,
    prog: Box<glprog>,
}

pub struct Program {
    data: ProgramAttribs,
    pub shapes: ShapeManager,
    pub gtexitman: TextureTargetManager,
    code: ProgramCode,
}

fn find_uniforms(ctx: &glctx, prog: &Box<glprog>, vars: &Vec<Rc<Source>>) -> HashMap<String,gluni> {
    let mut udata = HashMap::<String,gluni>::new();
    for v in vars {
        v.preget(ctx,prog,&mut udata);
    }
    udata
}

fn find_attribs(adata: &ArenaData, vars: &Vec<Rc<Source>>) 
                        -> (Vec<Box<Object>>,HashMap<String,usize>) {
    let mut attribs = Vec::<Box<Object>>::new();
    let mut attrib_names = HashMap::<String,usize>::new();
    for v in vars {
        if let Some((name,value)) = v.make_attribs(adata) {
            let loc = attribs.len();
            attribs.push(value);
            if let Some(name) = name {
                attrib_names.insert(name.to_string(),loc);
            }
        }
    }
    (attribs,attrib_names)
}

impl ProgramCode {
    pub fn set_attribute(&self, ctx: &glctx, name: &str, buf: &glbuf, 
                         idx_buf: &glbuf, step: u8) {
        let prog = &self.prog;
        let loc = ctx.get_attrib_location(prog,name) as u32;
        ctx.enable_vertex_attrib_array(loc);
        ctx.bind_buffer(glctx::ARRAY_BUFFER,Some(buf));
        ctx.bind_buffer(glctx::ELEMENT_ARRAY_BUFFER,Some(idx_buf));
        ctx.vertex_attrib_pointer(loc, step as i32, glctx::FLOAT, false, 0, 0);
    }
    
    pub fn set_uniform_1i(&self, ctx: &glctx, name:&str, value: i32) {
        if let Some(loc) = self.uniforms.get(name) {
            ctx.uniform1i(Some(&loc),value);
        }
    }
    
    pub fn set_uniform_1f(&self, ctx: &glctx, name:&str, value: f32) {
        if let Some(loc) = self.uniforms.get(name) {
            ctx.uniform1f(Some(&loc),value);
        }
    }
    
    pub fn set_uniform_2f(&self, ctx: &glctx, name:&str, value: [f32;2]) {
        if let Some(loc) = self.uniforms.get(name) {
            ctx.uniform2f(Some(&loc),value[0],value[1]);
        }
    }
}

impl ProgramAttribs {
    pub fn advance(&mut self,amt: i32) { self.indices += amt; }    

    pub fn add_attrib_data(&mut self, name: &str, values: &[&Input]) {
        let loc = self.attrib_names[name];
        self.attribs[loc].add_data(values);
    }
}

impl Program {
    pub fn new(adata: &ArenaData, src: &ProgramSource) -> Program {
        let prog = Box::new(src.prog(adata));
        adata.ctx.use_program(Some(&prog));
        let uniforms = find_uniforms(&adata.ctx,&prog,&src.uniforms);
        let (attribs,attrib_names) = find_attribs(adata,&src.uniforms);
        Program {
            gtexitman: TextureTargetManager::new(),
            shapes: ShapeManager::new(),
            data: ProgramAttribs {
                attribs, attrib_names,
                indices: 0
            },
            code: ProgramCode {
                prog, uniforms,
            }
        }
    }
  
    pub fn draw(&mut self, adata: &ArenaData, stage:&Stage) {
        self.link(adata,stage,&adata.dims);
        if self.data.indices > 0 {
            adata.ctx.draw_arrays(glctx::TRIANGLES,0,self.data.indices);
            
            adata.ctx.draw_elements(glctx::TRIANGLES,self.data.indices,
                                    glctx::UNSIGNED_SHORT,0);
        }
    }
    
    pub fn link(&mut self, adata : &ArenaData,  stage: &Stage, dims: &ArenaDims) {
        let ctx = &adata.ctx;
        let prog = &self.code.prog;
        ctx.use_program(Some(&prog));
        for a in &self.data.attribs {
            a.link(adata,&self.code,stage,dims);
        }
    }
    
    pub fn populate(&mut self, adata: &mut ArenaData) {
        self.gtexitman.draw(&mut self.data,adata);
        self.gtexitman.clear();
        self.shapes.draw(&mut self.data,adata);
        self.shapes.clear();
        for a in &mut self.data.attribs {
            a.populate(adata);
        }
    }
}
