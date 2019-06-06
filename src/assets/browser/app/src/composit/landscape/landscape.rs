/* This is a placeholder for more sophisticated y-coodinate handling
 * in future. In general a Landscape will include multiple plots and
 * maybe other things, in different roles.
 */

use super::Plot;

pub struct Landscape {
    name: String,
    lid: Option<usize>,
    plot: Option<Plot>,
}

impl Landscape {
    pub(in super) fn new(name: &str) -> Landscape {
        Landscape { plot: None, lid: None, name: name.to_string() }
    }
    
    pub(in super) fn set_lid(&mut self, lid: usize) {
        self.lid = Some(lid);
    }
    
    pub fn get_name(&self) -> &str { &self.name }
    
    pub fn get_plot(&self) -> &Plot {
        &self.plot.as_ref().unwrap()
    }
    
    pub fn set_plot(&mut self, plot: Plot) {
        self.plot = Some(plot);
    }
    
    pub fn get_low_watermark(&self) -> Option<i32> {
        self.plot.as_ref().map(|x| x.get_low_watermark())
    }
}
