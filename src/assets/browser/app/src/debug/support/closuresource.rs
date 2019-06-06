use std::rc::Rc;

use composit::{ Source, Leaf, ActiveSource };
use model::train::PartyResponses;
use model::shape::ShapeSpec;

pub struct ClosureSourceImpl {
    f: Box<Fn(&mut PartyResponses,&Leaf)>,
    delay: f32
}

#[derive(Clone)]
pub struct ClosureSource(Rc<ClosureSourceImpl>);

impl ClosureSource {
    pub fn new<F>(delay: f32, f: F) -> ClosureSource 
                           where F: Fn(&mut PartyResponses,&Leaf) + 'static {
        ClosureSource(Rc::new(ClosureSourceImpl{ f: Box::new(f), delay }))
    }
}

impl Source for ClosureSource {
    fn populate(&self, acs: &ActiveSource, mut lc: PartyResponses, leaf: &Leaf) {
        let cs = self.clone();
        let leaf = leaf.clone();
        (cs.0.f)(&mut lc,&leaf);
    }
}

pub fn closure_add_opt(lcb: &mut PartyResponses, s: &Option<ShapeSpec>) {
    if let Some(s) = s {
        closure_add(lcb,s);
    }
}
pub fn closure_add(lcb: &mut PartyResponses, s: &ShapeSpec) {
    let srb = lcb.get_mut(&None).unwrap();
    srb.add_shape(s.clone());
}

pub fn closure_done(lcb: &mut PartyResponses) {
    lcb.done();
}
