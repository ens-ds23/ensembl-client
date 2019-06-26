/* Lazily builds a SourceManager from CombinedSources. In other words
 * there is an entry here for each component (track), by name.
 */

use std::collections::HashMap;

use composit::{ ActiveSource, AllLandscapes, SourceManager };
use data::{ BackendConfig, HttpXferClerk };
use drivers::zmenu::ZMenuRegistry;
use tácode::Tácode;
use super::build_combined_source;

pub struct CombinedSourceManager {
    tc: Tácode,
    sources: HashMap<String,Option<ActiveSource>>,
    config: BackendConfig,
    als: AllLandscapes,
    xf: HttpXferClerk,
    zmr: ZMenuRegistry
}

impl CombinedSourceManager {
    pub fn new(tc: &Tácode, config: &BackendConfig, zmr: &ZMenuRegistry,
               als: &AllLandscapes, xf: &HttpXferClerk) -> CombinedSourceManager {
        CombinedSourceManager {
            zmr: zmr.clone(),
            tc: tc.clone(),
            config: config.clone(),
            als: als.clone(),
            sources: HashMap::<String,Option<ActiveSource>>::new(),
            xf: xf.clone()
        }
    }
}

impl SourceManager for CombinedSourceManager {
    fn get_component(&mut self, name: &str) -> Option<ActiveSource> {
        if !self.sources.contains_key(name) {
            let tc = self.tc.clone();
            let source = build_combined_source(&tc,&self.config,&self.zmr,&mut self.als,&self.xf,name);
            self.sources.insert(name.to_string(),source);
        }
        self.sources[name].clone()
    }
}
