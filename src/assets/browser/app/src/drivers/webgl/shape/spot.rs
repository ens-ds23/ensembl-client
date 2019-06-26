use std::collections::HashMap;

use super::super::program::{ ProgramAttribs, DataGroupIndex, ProgramType, PTSkin };
use types::Colour;

pub struct Spot {
    group: HashMap<ProgramType,HashMap<Colour,DataGroupIndex>>
}

impl Spot {
    pub fn new() -> Spot {
        Spot {
            group: HashMap::<ProgramType,HashMap<Colour,DataGroupIndex>>::new(),
        }
    }

    pub fn get_group(&mut self, geom: &mut ProgramAttribs, colour: &Colour) -> DataGroupIndex {
        *self.group.entry(*geom.prog_type()).or_insert_with(||
            HashMap::<Colour,DataGroupIndex>::new()
        ).entry(*colour).or_insert_with(|| {
            geom.new_group()
        })
    }

    pub fn into_objects(&mut self, geom: &mut ProgramAttribs) {
        let geom_name = *geom.prog_type();
        if geom_name.2 == PTSkin::Spot {
            if let Some(obj) = geom.get_object("uColour") {
                if let Some(m) = self.group.get(&geom_name) {
                    for (c,g) in m {
                        obj.set_uniform(Some(*g),c.to_uniform());
                    }
                }
            }
        }
    }
}
