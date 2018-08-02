use geometry::{
    Geometry,
    GeomContext,
    GTypeHolder,
    GTypeAttrib,
    GType,
    GTypeCanvasTexture,
    GTypeTicket,
};

use webgl_rendering_context::{
    WebGLRenderingContext as glctx,
    WebGLBuffer as glbuf,
    WebGLProgram as glprog
};

use geometry;

use alloc::{
    Ticket,
    Allocator
};

use arena::{
    ArenaData,
    Stage
};

use canvasutil::FCFont;
use canvasutil::FlatCanvas;
use canvasutil;

use std::cell::RefCell;
use std::rc::Rc;

const V_SRC : &str = "
attribute vec2 aVertexPosition;
attribute vec2 aOrigin;
attribute vec2 aTextureCoord;

uniform float uAspect;
uniform float uStageHpos;
uniform float uStageVpos;
uniform float uStageZoom;

varying highp vec2 vTextureCoord;

void main() {
    gl_Position = vec4(
        (aOrigin.x - uStageHpos) * uStageZoom + aVertexPosition.x,
        (aOrigin.y - uStageVpos) + aVertexPosition.y,
        0.0, 1.0
    );
    vTextureCoord = aTextureCoord;
}
";

const F_SRC : &str = "
varying highp vec2 vTextureCoord;

uniform sampler2D uSampler;

void main() {
      gl_FragColor = texture2D(uSampler, vTextureCoord);
}
";

pub struct TextGeometry {
    std: GeomContext,
    requests: Vec<TextReq>,
    pos: GTypeAttrib,
    origin: GTypeAttrib,
    coord: GTypeAttrib,
    sampler: GTypeCanvasTexture,
    tickets: GTypeTicket,
}

pub struct TextReq {
    origin: [f32;2],
    width: u32,
    height: u32,
    chars: String,
    font: FCFont,
    ticket: Ticket
}

impl TextReq {
    fn new(adata: &mut ArenaData, chars: &str, origin: &[f32;2], font: &FCFont) -> TextReq {
        let flat = &adata.flat;
        let (width, height) = flat.measure(chars,font);
        let flat_alloc = &mut adata.flat_alloc;
        TextReq {
            width, height, origin: *origin,
            ticket: flat_alloc.request(width,height),
            font: font.clone(),
            chars: chars.to_string()
        }
    }
        
    fn insert(&self, adata: &ArenaData, x: u32, y: u32) {
        adata.flat.text(&self.chars,x,y,&self.font);
    }
}

impl GTypeHolder for TextGeometry {
    fn gtypes(&mut self) -> (&GeomContext,Vec<&mut GType>) {
        (&self.std,
        vec! { &mut self.sampler, &mut self.pos,
               &mut self.origin, &mut self.coord,
               &mut self.tickets })
    }
}

impl TextGeometry {
    pub fn new(adata: Rc<RefCell<ArenaData>>) -> TextGeometry {                   
        TextGeometry {
            std: GeomContext::new(adata.clone(),&V_SRC,&F_SRC),
            pos:    GTypeAttrib::new(&adata.borrow(),"aVertexPosition",2,1),
            origin: GTypeAttrib::new(&adata.borrow(),"aOrigin",2,3),
            coord:  GTypeAttrib::new(&adata.borrow(),"aTextureCoord",2,1),
            sampler: GTypeCanvasTexture::new("uSampler",0),
            tickets: GTypeTicket::new(),
            requests: Vec::<TextReq>::new(),
        }
    }
            
    fn triangle(&mut self,origin:&[f32;2],points:&[f32;6],tex_points:&[f32;6]) {
        self.pos.add(points);
        self.origin.add(origin);
        self.coord.add(tex_points);
        self.std.advance(3);
    }
    
    fn rectangle(&mut self,origin:&[f32;2],p:&[f32;4],t:&[f32;4]) {
        self.triangle(origin,&[p[0],p[1],p[2],p[1],p[0],p[3]],
                             &[t[0],t[1],t[2],t[1],t[0],t[3]]);
        self.triangle(origin,&[p[2],p[3],p[0],p[3],p[2],p[1]],
                             &[t[2],t[3],t[0],t[3],t[2],t[1]]);
    }
    
    fn prepopulate(&mut self) {
        let adatac = self.std.get_adata();
        let mut adata = adatac.borrow_mut();
        for req in &self.requests {
            let (x,y) = adata.flat_alloc.position(&req.ticket);
            adata.flat.text(&req.chars,x,y,&req.font);
        }
        let flat = &adata.flat;
        let mut data = Vec::<([f32;2],[f32;4],[f32;4])>::new();
        for req in &self.requests {
            let nudge = adata.nudge((req.origin[0],req.origin[1]));
            let p = [
                nudge.0, nudge.1, 
                nudge.0 + adata.prop_x(req.width),
                nudge.1 + adata.prop_y(req.height)
            ];
            let (x,y) = adata.flat_alloc.position(&req.ticket);
            let t = [flat.prop_x(x), flat.prop_y(y + req.height),
                     flat.prop_x(x + req.width), flat.prop_y(y)];
            data.push((req.origin,p,t));
        }
        for (origin,p,t) in data {
            self.rectangle(&origin,&p,&t);
        }
        self.requests.clear();
    }
    
    pub fn text(&mut self,origin:&[f32;2],text: &str,font: &FCFont) {
        let adatac = self.std.get_adata();
        let mut adata = adatac.borrow_mut();
        let req = TextReq::new(&mut adata,text,origin,font);
        self.requests.push(req);
    }
}

impl Geometry for TextGeometry {
    fn populate(&mut self) {
        self.prepopulate();
        geometry::populate(self);
    }
    fn draw(&mut self,stage:&Stage) { geometry::draw(self,stage); }
}
