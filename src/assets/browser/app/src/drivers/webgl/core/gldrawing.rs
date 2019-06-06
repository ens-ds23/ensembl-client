use composit::SourceResponseData;
use super::super::drawing::{ Drawing, CarriageCanvases };
use super::GLProgInstances;
use super::super::shape::GLShape;

pub struct GLDrawing {
    drawings: Vec<Option<Drawing>>,
    sr: SourceResponseData,
}

impl GLDrawing {
    pub fn new(sr: SourceResponseData) -> GLDrawing {
        GLDrawing { 
            sr, 
            drawings: Vec::<Option<Drawing>>::new(),
        }
    }
    
    #[allow(unused)]
    pub fn size(&self) -> usize { self.sr.get_shapes().len() }
    
    pub fn redraw(&mut self, ds: &mut CarriageCanvases) {
        self.drawings.clear();
        for mut s in self.sr.get_shapes() {
            if let Some(a) = s.get_artist() {
                let ocm = a.select_canvas(ds);
                self.drawings.push(Some(ocm.add_request(a)));
            } else {
                self.drawings.push(None);
            }
        }
    }

    pub fn into_objects(&mut self, e: &mut GLProgInstances) {
        let mut di = self.drawings.iter();
        for mut s in self.sr.get_shapes().iter() {
            let d = di.next();
            let geom_name = s.get_geometry();
            let (progs,data) = e.get_progs_data();
            if let Some(geom) = progs.map.get_mut(&geom_name) {
                let artwork = d.unwrap().as_ref().map(|r| r.artwork(data));
                s.into_objects(&mut geom.data,artwork,data);
            }
        }
    }
}
