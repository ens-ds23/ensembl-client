use composit::{ Compositor, Stage, Leaf };
use types::Dot;

use model::train::{ Traveller, TravellerResponse };
use drivers::zmenu::ZMenuLeaf;

pub trait Printer {
    /* Print one run of objects from compositor with given stage and
     * state.
     */
    fn print(&mut self, stage: &Stage, compo: &mut Compositor);
    
    /* Redraw one carriage */
    fn redraw_carriage(&mut self, leaf: &Leaf);
    
    /* Finished with printer */
    fn destroy(&mut self);
    
    /* Set your size to this */
    fn set_size(&mut self, s: Dot<f64,f64>);
    
    /* no recent resizes, etc */
    fn settle(&mut self);
    
    /* How much size is available to expand into, should you be
     * requested to do so?
     */
    fn get_available_size(&self) -> Dot<f64,f64>;
    
    fn add_leaf(&mut self, leaf: &Leaf);
    fn remove_leaf(&mut self, leaf: &Leaf);
    fn set_current(&mut self, leaf: &Leaf);
    fn make_traveller_response(&mut self, leaf: &Leaf) -> Box<TravellerResponse>;
}
