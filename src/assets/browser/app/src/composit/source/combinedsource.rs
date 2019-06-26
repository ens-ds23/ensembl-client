use std::collections::HashMap;
use std::rc::Rc;

use composit::{ 
    ActiveSource, Leaf, Plot, Source,
    StateAtom, AllLandscapes
};
use data::{ BackendConfig, HttpXferClerk };
use debug::{ add_debug_sources };
use drivers::zmenu::ZMenuRegistry;
use composit::source::SourceResponse;
use tácode::{ Tácode, TáSource };

const TOP : i32 = 50;
const PITCH : i32 = 63;

/* A CombinedSource is a single *track* but it may come from multiple
 * sources depending on the stick (eg species). At the moment we only
 * support a single backend source along with local debug sources but
 * this is where any additional logic would go.
 */
pub struct CombinedSource {
    backend_source: Box<Source>,
    per_stick_sources: HashMap<String,Box<Source>>,
}

impl CombinedSource {
    pub fn new(backend_source: Box<Source>) -> CombinedSource {
        CombinedSource {
            backend_source,
            per_stick_sources: HashMap::<String,Box<Source>>::new()
        }
    }
    
    pub fn add_per_stick(&mut self, name: &str, source: Box<Source>) {
        let name = name.to_string();
        self.per_stick_sources.insert(name.clone(),source);
    }
}

impl Source for CombinedSource {
    fn request_data(&self, acs: &ActiveSource, lc: SourceResponse, leaf: &Leaf) {
        let stick_name = leaf.get_stick().get_name();
        if let Some(source) = self.per_stick_sources.get(&stick_name) {
            source.request_data(acs,lc,leaf);
        } else {
            self.backend_source.request_data(acs,lc,leaf);
        }
    }
}

pub fn build_combined_source(tc: &Tácode, config: &BackendConfig, zmr: &ZMenuRegistry, als: &mut AllLandscapes, xf: &HttpXferClerk, type_name: &str) -> Option<ActiveSource> {
    let lid = als.allocate(type_name);
    let cfg_track = config.get_track(type_name);
    let y_pos = cfg_track.map(|t| t.get_position()).unwrap_or(-1);
    let letter = cfg_track.map(|t| t.get_letter()).unwrap_or("");
    let plot = Plot::new(y_pos*PITCH+TOP,PITCH,letter.to_string(),y_pos!=-1);
    als.with(lid, |ls| ls.set_plot(plot) );
    let backend = TáSource::new(tc,Box::new(xf.clone()),type_name,lid,config);
    let mut combined = CombinedSource::new(Box::new(backend));
    add_debug_sources(&mut combined,type_name);
    let mut act = ActiveSource::new(type_name,Rc::new(combined),zmr,als,lid);
    act.new_part(None,Rc::new(StateAtom::new(&type_name)));
    let none = vec!{};
    let parts = cfg_track.map(|t| t.get_parts()).unwrap_or(&none);
    for part in parts {
        let state_name = format!("{}:{}",type_name,part);
        act.new_part(Some(part),Rc::new(StateAtom::new(&state_name)));
    }
    Some(act)
}
