use model::shape::ShapeSpec;

pub struct SourceResponseData {
    shapes: Vec<ShapeSpec>
}

impl SourceResponseData {
    pub fn new() -> SourceResponseData {
        SourceResponseData {
            shapes: Vec::<ShapeSpec>::new()
        }
    }

    pub fn expect(&mut self, amt: usize) {
        self.shapes.reserve(amt);
    }
    
    pub fn add_shape(&mut self, item: ShapeSpec) {
        self.shapes.push(item);
    }

    pub fn get_shapes(&self) -> &Vec<ShapeSpec> {
        &self.shapes
    }
        
    pub fn size(&self) -> usize {
        self.shapes.len()
    }
}
