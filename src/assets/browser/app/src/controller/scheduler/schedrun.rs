pub struct SchedRun {
    productive: bool,
    available: f64
}

impl SchedRun {
    pub(in super) fn new(available: f64) -> SchedRun {
        SchedRun {
            productive: true,
            available
        }
    }
    
    pub fn unproductive(&mut self) {
        self.productive = false;
    }
    
    pub fn available(&self) -> f64 {
        self.available
    }
    
    pub(in super) fn was_productive(&self) -> bool {
        self.productive
    }
}
