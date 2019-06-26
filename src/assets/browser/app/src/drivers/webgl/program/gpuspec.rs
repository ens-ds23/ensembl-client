use std::cmp::{ min, Ordering };
use std::collections::HashMap;

use dom::webgl::{
    WebGLRenderingContext as glctx,
    GLenum,
};

#[derive(PartialEq,Clone,Copy)]
pub enum Precision {
    Float(i32,i32),
    Int(i32)
}

#[derive(PartialEq,Eq,Clone,Copy,Hash)]
pub enum Arity {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Sampler2D,
}

impl Arity {
    pub fn to_num(&self) -> u8 {
        *ARITY.get(self).unwrap_or(&0)
    }
}

impl PartialOrd for Precision {
    fn partial_cmp(&self, other: &Precision) -> Option<Ordering> {
        match (self,other) {
            (Precision::Float(ap,ar),Precision::Float(bp,br)) => {
                if (ap<bp && ar<=br) || (ar<br && ap<=bp) {
                    Some(Ordering::Less)
                } else if (bp<ap && br<=ar) || (br<ar && bp<=ap) {
                    Some(Ordering::Greater)
                } else if other == self {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            },
            (Precision::Float(_,_),Precision::Int(_)) =>
                Some(Ordering::Greater),
            (Precision::Int(_),Precision::Float(_,_)) =>
                Some(Ordering::Less),
            (Precision::Int(a),Precision::Int(b)) =>
                Some(a.cmp(&b))
        }
    }
}

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
pub enum GLSize {
    FloatHigh,
    FloatMed,
    FloatLow,
    IntHigh,
    IntMed,
    IntLow
}

lazy_static! {
    static ref PREC_STR : HashMap<GLSize,&'static str> = hashmap! {
        GLSize::FloatHigh => "highp", GLSize::IntHigh => "highp",
        GLSize::FloatMed => "mediump", GLSize::IntMed => "mediump",
        GLSize::FloatLow => "lowp", GLSize::IntLow => "lowp"
    };
    
    static ref IS_INT : HashMap<GLSize,bool> = hashmap! {
        GLSize::IntHigh => true,
        GLSize::IntMed => true,
        GLSize::IntLow => true
    };
    
    static ref TYPES : HashMap<Arity,[&'static str;2]> = hashmap! {
        Arity::Scalar => ["float","int"],
        Arity::Vec2 => ["vec2","ivec2"],
        Arity::Vec3 => ["vec3","ivec3"],
        Arity::Vec4 => ["vec4","ivec4"],
        Arity::Sampler2D => ["sampler2D",""]
    };
    
    static ref ARITY : HashMap<Arity,u8> = hashmap! {
        Arity::Scalar => 1,
        Arity::Vec2 => 2,
        Arity::Vec3 => 3,
        Arity::Vec4 => 4
    };
}

impl GLSize {
    pub fn as_string(&self, size: Arity) -> String {
        let idx = if self.is_int() { 1 } else { 0 };
        format!("{} {}",PREC_STR[self],TYPES[&size][idx]).to_string()
    }

    fn is_int(&self) -> bool {
        *IS_INT.get(self).unwrap_or(&false)
    }
}

const ORDER: &[GLSize] = &[
    GLSize::IntLow, GLSize::IntMed, GLSize::IntHigh,
    GLSize::FloatLow, GLSize::FloatMed, GLSize::FloatHigh,
];

pub struct GPUSpecImpl {
    vert_precs: HashMap<GLSize,Precision>,
    frag_precs: HashMap<GLSize,Precision>
}

pub struct GPUSpec(Option<GPUSpecImpl>);

impl GPUSpec {
    pub fn new() -> GPUSpec { GPUSpec(None) }
    
    pub fn populate(&mut self, ctx: &glctx) {
        self.0 = Some(GPUSpecImpl::new(ctx));
    }
    
    pub fn best_vert(&self, want: &Precision) -> GLSize {
        self.0.as_ref().unwrap().best_vert(want)
    }

    pub fn best_frag(&self, want: &Precision) -> GLSize {
        self.0.as_ref().unwrap().best_frag(want)
    }
}

fn get_prec(out: &mut HashMap<GLSize,Precision>, ctx: &glctx,
            shader_en: GLenum, size: GLSize, type_en: GLenum) {
    let prec = ctx.get_shader_precision_format(shader_en,type_en);
    if let Some(prec) = prec {
        let range = min(prec.range_min(),prec.range_max());
        let val = if size.is_int() {
            Precision::Int(range)
        } else {
            Precision::Float(prec.precision(),range)
        };
        out.insert(size,val);
    }
}

fn get_precisions(ctx: &glctx, shader_en: GLenum) 
                                        -> HashMap<GLSize,Precision> {
    let mut out = HashMap::<GLSize,Precision>::new();
    get_prec(&mut out,ctx,shader_en,GLSize::FloatHigh,glctx::HIGH_FLOAT);
    get_prec(&mut out,ctx,shader_en,GLSize::FloatMed,glctx::MEDIUM_FLOAT);
    get_prec(&mut out,ctx,shader_en,GLSize::FloatLow,glctx::LOW_FLOAT);
    get_prec(&mut out,ctx,shader_en,GLSize::IntHigh,glctx::HIGH_INT);
    get_prec(&mut out,ctx,shader_en,GLSize::IntMed,glctx::MEDIUM_INT);
    get_prec(&mut out,ctx,shader_en,GLSize::IntLow,glctx::LOW_INT);
    out
}

impl GPUSpecImpl {
    pub fn new(ctx: &glctx) -> GPUSpecImpl {
        GPUSpecImpl {
            vert_precs: get_precisions(ctx,glctx::VERTEX_SHADER),
            frag_precs: get_precisions(ctx,glctx::FRAGMENT_SHADER)
        }
    }
    
    fn best(&self, want: &Precision,
            precs: &HashMap<GLSize,Precision>) -> GLSize {
        for p in ORDER {
            if precs[p] > *want { return *p; }
        }
        return *ORDER.last().unwrap();
    }
    
    pub fn best_vert(&self, want: &Precision) -> GLSize {
        self.best(want,&self.vert_precs)
    }
    
    pub fn best_frag(&self, want: &Precision) -> GLSize {
        self.best(want,&self.frag_precs)
    }    
}
