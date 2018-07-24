use arena::{
    Geometry,
    ArenaData,
    StdGeometry,
    GeomBuf,
    Stage,
};

use std::cell::RefCell;
use std::rc::Rc;

const V_SRC : &str = "
attribute vec2 aVertexPosition;
attribute vec3 aVertexColour;

uniform float uAspect;
uniform float uStageHpos;
uniform float uStageVpos;
uniform float uStageZoom;

varying lowp vec3 vColour;

void main() {
     gl_Position = vec4(
          (aVertexPosition.x - uStageHpos) * uStageZoom,
          (aVertexPosition.y - uStageVpos),
          0.0, 1.0
    );
    vColour = aVertexColour;
}
";

const F_SRC : &str = "
varying lowp vec3 vColour;

void main() {
      gl_FragColor = vec4(vColour, 1.0);
}
";

pub struct HoscGeometry {
    std : StdGeometry,
    points : GeomBuf,
    colours : GeomBuf,
}

impl HoscGeometry {
    pub fn new(adata: Rc<RefCell<ArenaData>>) -> HoscGeometry {
        let ctx = &adata.borrow().ctx;
        let std = StdGeometry::new(adata.clone(),&V_SRC,&F_SRC);
        HoscGeometry {
            std,
            points: GeomBuf::new(&ctx,"aVertexPosition",2),
            colours: GeomBuf::new(&ctx,"aVertexColour",3),
        }
    }

    pub fn triangle(&mut self,points:[f32;6],colour:[f32;3]) {
        self.points.add(&points,1);
        self.colours.add(&colour,3);
        self.std.indices = self.std.indices + 3
    }
}

impl Geometry for HoscGeometry {
    fn populate(&mut self) {
        self.std.select();
        self.points.populate(&self.std);
        self.colours.populate(&self.std);
    }

    fn draw(&self) {
        self.std.select();
        self.points.link(&self.std);
        self.colours.link(&self.std);
        self.std.draw_triangles();
    }
    
    fn perspective(&self,stage:&Stage) {
        self.std.perspective(stage);
    }
}
