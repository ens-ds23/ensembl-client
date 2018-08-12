use geometry::{
    Geometry,
    GLProgram,
    GTypeAttrib,
    GType,
    PCoord,
};

use geometry::wglprog::{
    GLSource,
    Statement,
    shader_solid,
};

use geometry;
use geometry::wglprog;

use webgl_rendering_context::{
    WebGLRenderingContext as glctx,
    WebGLProgram as glprog,
};

use arena::{
    ArenaData,
    ArenaDims,
    Stage
};

pub struct FixGeometry {
    std : GLProgram,
    pos: GTypeAttrib,
    colour: GTypeAttrib,
}

impl Geometry for FixGeometry {
    fn populate(&mut self, adata: &mut ArenaData) { geometry::populate(self,adata); }
    fn draw(&mut self, adata: &mut ArenaData,stage:&Stage) { geometry::draw(self,adata,stage); }

    fn gtypes(&mut self) -> (&GLProgram,Vec<&mut GType>) {
        (&self.std,vec! { &mut self.pos, &mut self.colour })
    }
    
    fn restage(&mut self, ctx: &glctx, prog: &glprog, stage: &Stage, dims: &ArenaDims) {
        self.std.set_uniform_1f(&ctx,"uStageHpos",stage.pos.0);
        self.std.set_uniform_1f(&ctx,"uStageVpos",stage.pos.1 + (dims.height_px as f32/2.));
        self.std.set_uniform_1f(&ctx,"uStageZoom",stage.zoom);
        self.std.set_uniform_1f(&ctx,"uAspect",dims.aspect);
        self.std.set_uniform_2f(&ctx,"uSize",[
            dims.width_px as f32/2.,
            dims.height_px as f32/2.]);
    }
}

impl FixGeometry {
    pub fn new(adata: &ArenaData) -> FixGeometry {
        let source = shader_solid(&GLSource::new(vec! {
            Statement::new_vertex("
                gl_Position = vec4(aVertexPosition.x / uSize.x - 1.0,
                                   aVertexPosition.y / uSize.y - 1.0,
                                   0.0, 1.0)")

        }));
        FixGeometry {
            std: GLProgram::new(adata,&source),
            pos: GTypeAttrib::new(adata,"aVertexPosition",2,1),
            colour: GTypeAttrib::new(adata,"aVertexColour",3,3),
        }
    }

    pub fn triangle(&mut self,points:&[PCoord;3],colour:&[f32;3]) {
        self.pos.add_px(points);
        self.colour.add(colour);
        self.std.advance(3);
    }
    
    pub fn rectangle(&mut self,p:&[PCoord;2],colour:&[f32;3]) {
        let mix = &p[0].mix(p[1]);
        self.triangle(&[p[0], mix.1, mix.0],colour);
        self.triangle(&[p[1], mix.0, mix.1],colour);
    }
}
