use std::rc::Rc;
use std::collections::HashMap;

pub trait StateExpr {
    fn is_on(&self, _m: &StateManager) -> bool { false }
}

pub struct StateFixed(bool);

impl StateExpr for StateFixed {
    fn is_on(&self, _m: &StateManager) -> bool { self.0 }
}

pub struct StateAtom {
    name: String
}

impl StateAtom {
    pub fn new(name: &str) -> StateAtom {
        StateAtom {
            name: name.to_string()
        }
    }
}

impl StateExpr for StateAtom {
    fn is_on(&self, m: &StateManager) -> bool {
        m.get_atom_state(&self.name)
    }
}

#[allow(unused)]
pub enum StateOp {
    And(Rc<StateExpr>,Rc<StateExpr>),
    Or(Rc<StateExpr>,Rc<StateExpr>),
    Not(Rc<StateExpr>)
}

impl StateExpr for StateOp {
    fn is_on(&self, m: &StateManager) -> bool {
        match self {
        StateOp::And(a,b) =>
            a.is_on(m) & b.is_on(m),
        StateOp::Or(a,b) =>
            a.is_on(m) | b.is_on(m),
        StateOp::Not(a) =>
            !a.is_on(m)
        }
    }    
}

#[allow(unused,dead_code)]
pub struct StateManager {
    atoms: HashMap<String,bool>,
    exprs: HashMap<String,Rc<StateExpr>>,
    changed: bool
}

#[allow(unused)]
impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            atoms: HashMap::<String,bool>::new(),
            exprs: HashMap::<String,Rc<StateExpr>>::new(),
            changed: false
        }
    }
    
    pub fn get_atom(&self, name: &str) -> StateAtom {
        StateAtom::new(name)
    }
    
    fn get_atom_state(&self, name: &str) -> bool {
        *self.atoms.get(name).unwrap_or(&false)
    }
    
    pub fn set_atom_state(&mut self, name: &str, val: bool) {
       self.atoms.insert(name.to_string(),val); 
       self.changed = true;
    }
    
    fn has_changed(&self) -> bool { self.changed }
    fn reset_changed(&mut self) { self.changed = false; }
}
