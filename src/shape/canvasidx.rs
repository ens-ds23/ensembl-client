use std::collections::HashMap;

use program::{ ProgramAttribs, DataGroup, ProgramType, PTSkin, CanvasWeave };
use program::UniformValue;

pub struct CanvasIdx {
    group: HashMap<ProgramType,HashMap<CanvasWeave,DataGroup>>,
    glindexes: HashMap<CanvasWeave,u32>
}

impl CanvasIdx {
    pub fn new(glindexes: HashMap<CanvasWeave,u32>) -> CanvasIdx {
        CanvasIdx {
            group: HashMap::<ProgramType,HashMap<CanvasWeave,DataGroup>>::new(),
            glindexes
        }
    }

    pub fn get_group(&mut self, geom: &mut ProgramAttribs, weave: &CanvasWeave) -> DataGroup {
        *self.group.entry(*geom.prog_type()).or_insert_with(||
            HashMap::<CanvasWeave,DataGroup>::new()
        ).entry(*weave).or_insert_with(|| {
            geom.new_group()
        })
    }

    pub fn into_objects(&mut self, geom: &mut ProgramAttribs) {
        let geom_name = *geom.prog_type();
        if geom_name.2 == PTSkin::Texture {
            if let Some(obj) = geom.get_object("uSampler") {
                if let Some(m) = self.group.get(&geom_name) {
                    for (w,g) in m {
                        let idx = self.glindexes[w] as i32;
                        obj.set_uniform(Some(*g),UniformValue::Int(idx));
                    }
                }
            }
        }
    }
}
