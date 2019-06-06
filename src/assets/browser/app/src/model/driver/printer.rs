use composit::{ Compositor, Stage, Leaf };
use types::Dot;

use model::driver::SourceResponse;

pub trait Printer {
    /* Print one run of objects from compositor with given stage and
     * state.
     */
    fn print(&mut self, stage: &Stage, compo: &mut Compositor);
    
    /* Finished with printer */
    fn destroy(&mut self);
    
    /* Set your size to this */
    fn set_size(&mut self, s: Dot<i32,i32>);
    
    /* no recent resizes, etc */
    fn settle(&mut self);
    
    /* How much size is available to expand into, should you be
     * requested to do so?
     */
    fn get_available_size(&self) -> Dot<i32,i32>;
    
    fn add_leaf(&mut self, leaf: &Leaf);
    fn remove_leaf(&mut self, leaf: &Leaf);
    fn set_current(&mut self, leaf: &Leaf);
    fn make_partial(&mut self, leaf: &Leaf) -> Box<SourceResponse>;
}
