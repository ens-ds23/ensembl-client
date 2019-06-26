use std::rc::Rc;

use dom::webgl::{
    WebGLRenderingContext as glctx,
    WebGLProgram as glprog,
};

use super::objects::{
    Object,
    ObjectAttrib,
    ObjectUniform,
    ObjectMain,
    ObjectCanvasTexture,
};

use super::gpuspec::{ Precision, Arity, GPUSpec };

#[derive(PartialEq,Clone,Eq)]
pub enum Phase {
    Vertex,
    Fragment,
}

pub trait Source {
    fn declare(&self, _adata: &GPUSpec, _phase: &Phase) -> String { String::new() }
    fn create(&self, _prog: Rc<glprog>) -> Option<(Option<&str>,Box<Object>)> {
        None
    }
    fn statement(&self, _phase: &Phase) -> String { String::new() }
}

pub struct Uniform {
    size: Arity,
    pub name: String,
    phase: Phase,
    prec: Precision,
}

impl Uniform {
    fn new(prec: &Precision, size: Arity, name: &str, phase: Phase) -> Rc<Uniform> {
        Rc::new(Uniform {
            size: size, name: name.to_string(), phase, 
            prec: *prec
        })
    }
    
    pub fn new_frag(prec: &Precision, size: Arity, name: &str) -> Rc<Uniform> {
        Uniform::new(prec,size, name, Phase::Fragment)
    }

    pub fn new_vert(prec: &Precision, size: Arity, name: &str) -> Rc<Uniform> {
        Uniform::new(prec,size, name, Phase::Vertex)
    }
}

impl Source for Uniform {
    fn declare(&self, gpuspec: &GPUSpec, phase: &Phase) -> String {
        if *phase == self.phase {
            let prec = match self.phase {
                Phase::Vertex =>
                    gpuspec.best_vert(&self.prec),
                Phase::Fragment =>
                    gpuspec.best_frag(&self.prec)
            };
            format!("uniform {} {};\n",
                prec.as_string(self.size),
                self.name).to_string()
        } else {
            String::new()
        }
    }

    fn create(&self, prog: Rc<glprog>) -> Option<(Option<&str>,Box<Object>)> {
        let gt = ObjectUniform::new(&prog,&self.name);
        Some((Some(&self.name),Box::new(gt)))
    }
}

#[derive(Clone)]
pub struct Attribute {
    size: Arity,
    prec: Precision,
    pub name: String,
}

impl Attribute {
    pub fn new(prec: &Precision, size: Arity, name: &str) -> Rc<Attribute> {
        Rc::new(Attribute {
            size, name: name.to_string(), prec: *prec
        })
    }
}

impl Source for Attribute {
    fn declare(&self, gpuspec: &GPUSpec, phase: &Phase) -> String {
        if *phase != Phase::Vertex { return String::new(); }
        format!("attribute {} {};\n",
            gpuspec.best_vert(&self.prec).as_string(self.size),
            self.name).to_string()
    }

    fn create(&self, prog: Rc<glprog>) -> Option<(Option<&str>,Box<Object>)> {
        let gt = ObjectAttrib::new(&prog,&self.name,self.size.to_num());
        Some((Some(&self.name),Box::new(gt)))
    }
}

#[derive(Clone)]
pub struct Varying {
    prec: Precision,
    size: Arity,
    name: String,
}

impl Varying {
    pub fn new(prec: &Precision, size: Arity, name: &str) -> Rc<Varying> {
        Rc::new(Varying {
            prec: *prec,
            size: size,
            name: name.to_string()
        })
    }
}

impl Source for Varying {
    fn declare(&self, gpuspec: &GPUSpec, _phase: &Phase) -> String {
        format!("varying {} {};\n",
            gpuspec.best_frag(&self.prec).as_string(self.size),
            self.name).to_string()
    }
}

#[derive(Clone)]
pub struct Statement {
    src: String,
    phase: Phase,
}

impl Statement {
    fn new(src: &str, phase: Phase) -> Rc<Statement> {
        Rc::new(Statement {
            src: src.to_string(),
            phase
        })
    }
    
    pub fn new_vert(src: &str) -> Rc<Statement> {
        Statement::new(src, Phase::Vertex)
    }

    pub fn new_frag(src: &str) -> Rc<Statement> {
        Statement::new(src, Phase::Fragment)
    }
}

impl Source for Statement {
    fn statement(&self, phase: &Phase) -> String {
        if *phase == self.phase {
            format!("{};\n",self.src)
        } else {
            String::new()
        }
    }
}

pub struct Canvas {
}

impl Canvas {
    pub fn new() -> Rc<Canvas> {
        Rc::new(Canvas { })
    }
}

impl Source for Canvas {
    fn create(&self, _prog: Rc<glprog>) -> Option<(Option<&str>,Box<Object>)> {
        Some((None,Box::new(ObjectCanvasTexture::new())))
    }
}

pub struct Main {
    method: u32
}

impl Main {
    pub fn new(method: u32) -> Rc<Main> {
        Rc::new(Main { method })
    }
}

impl Source for Main {
    fn create(&self, _prog: Rc<glprog>) -> Option<(Option<&str>,Box<Object>)> {
        Some((None,Box::new(ObjectMain::new(self.method))))
    }
}

fn declare(gpuspec: &GPUSpec, variables: &Vec<Rc<Source>>, phase: &Phase) -> String {
    let mut out = String::new();
    for v in variables {
        out += &v.declare(gpuspec,phase)[..];
    }
    out
}

fn statements(variables: &Vec<Rc<Source>>, phase: &Phase) -> String {
    let mut out = String::new();
    for v in variables {
        out += &v.statement(phase)[..];
    }
    out
}

pub struct ProgramSource {
    pub uniforms: Vec<Rc<Source>>
}

fn make_shader(ctx: &glctx, program: &glprog, kind: u32, src: &str) {
    let shader = ctx.create_shader(kind).unwrap();
    ctx.shader_source(&shader, src);
    ctx.compile_shader(&shader);
    ctx.attach_shader(program, &shader);
}


fn make_source(gpuspec: &GPUSpec, uniforms: &Vec<Rc<Source>>, frag: &Phase) -> String {
    format!("{}\n\nvoid main() {{\n{}\n}}",
        declare(gpuspec,uniforms,&frag),
        statements(uniforms,&frag))
}

impl ProgramSource {
    pub fn new(src: Vec<Rc<Source>>) -> ProgramSource {
        ProgramSource { uniforms: src }
    }

    pub fn prog(&self, gpuspec: &GPUSpec, ctx: &glctx) -> glprog {
        let prog = ctx.create_program().unwrap();
        let vertex = make_source(gpuspec,&self.uniforms,&Phase::Vertex);
        let fragment = make_source(gpuspec,&self.uniforms,&Phase::Fragment);
        make_shader(ctx,&prog,glctx::VERTEX_SHADER,&vertex);
        make_shader(ctx,&prog,glctx::FRAGMENT_SHADER,&fragment);
        bb_log!("webgl-programs","--- vertex ---\n{}",&vertex);
        bb_log!("webgl-programs","--- fragment ---\n{}",&fragment);
        ctx.link_program(&prog);        
        prog
    }
    
    pub fn merge(&self, other: &ProgramSource) -> ProgramSource {
        ProgramSource::new([
            &self.uniforms[..],
            &other.uniforms[..]
        ].concat())
    }
}
