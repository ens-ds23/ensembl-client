use arena::ArenaData;
use program::Program;

use geometry::common::{ shader_solid, shader_texture, PR_DEF };

use program::{
    ProgramSource,
    Statement,
    Uniform,
    Attribute,
};

fn stretch_prog() -> ProgramSource {
    ProgramSource::new(vec! {
        Uniform::new_vert("float","uStageHpos"),
        Uniform::new_vert("float","uStageVpos"),
        Uniform::new_vert("float","uStageZoom"),
        Uniform::new_vert("vec2","uSize"),
        Attribute::new(&PR_DEF,2,"aVertexPosition"),
        Statement::new_vert("
            gl_Position = vec4(
                (aVertexPosition.x - uStageHpos) * uStageZoom,
                - (aVertexPosition.y - uStageVpos) / uSize.y,
                0.0, 1.0)")
    })
}

pub fn stretch_geom(adata: &ArenaData) -> Program {
    Program::new(adata,&shader_solid(&stretch_prog()))
}

pub fn stretchtex_geom(adata: &ArenaData) -> Program {
    Program::new(adata,&shader_texture(&stretch_prog()))
}

