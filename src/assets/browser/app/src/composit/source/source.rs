use composit::{ Leaf, ActiveSource };
use composit::source::SourceResponse;

pub trait Source {
    fn request_data(&self, acs: &ActiveSource, lc: SourceResponse, leaf: &Leaf);
}
