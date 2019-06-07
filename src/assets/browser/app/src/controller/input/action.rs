use types::{ Move, Units, Axis, Dot, cdfraction, LEFT, RIGHT };
use controller::global::App;

#[derive(Debug,Clone)]
pub enum Action {
    Noop,
    Pos(Dot<f64,f64>,Option<f64>),
    PosRange(f64,f64,f64),
    Move(Move<f64,f64>),
    Zoom(f64),
    ZoomTo(f64),
    Resize(Dot<f64,f64>),
    AddComponent(String),
    SetStick(String),
    SetState(String,bool),
    Settled
}

fn exe_pos_event(app: &App, v: Dot<f64,f64>, prop: Option<f64>) {
    let prop = prop.unwrap_or(0.5);
    let v = app.with_stage(|s|
        Dot(s.pos_prop_bp_to_origin(v.0,prop),v.1)
    );
    let pos = app.with_stage(|s| { s.set_pos_middle(&v); s.get_pos_middle() });
    app.with_compo(|co| { co.set_position(pos.0); });
}

fn exe_pos_range_event(app: &App, x_start: f64, x_end: f64, y: f64) {
    let middle = Dot((x_start+x_end)/2.,y);
    let (pos,zoom) = app.with_stage(|s| { 
        s.set_screen_in_bp(x_end-x_start);
        s.set_pos_middle(&middle); 
        (s.get_pos_middle(),s.get_linear_zoom())
    });
    app.with_compo(|co| {
        co.set_zoom(zoom);
        co.set_position(pos.0);
    });
}

fn exe_move_event(app: &App, v: Move<f64,f64>) {
    let pos = app.with_stage(|s| {
        let v = match v.direction().0 {
            Axis::Horiz => v.convert(Units::Bases,s),
            Axis::Vert => v.convert(Units::Pixels,s),
            Axis::Zoom => v // TODO invalid pre-unification
        };
        s.inc_pos(&v);
        s.get_pos_middle()
    });
    app.with_compo(|co| {
        co.set_position(pos.0);
    });
}

fn exe_zoom_event(app: &App, z: f64, by: bool) {
    let middle = app.with_stage(|s| s.get_pos_middle().0);
    let z = app.with_stage(|s| {
        if by {
            s.inc_zoom(z);
        } else {
            s.set_zoom(z);
        }
        s.get_linear_zoom()
    });
    app.with_compo(|co| { co.set_zoom(z); co.set_position(middle); });
}

fn exe_resize(cg: &mut App, sz: Dot<f64,f64>) {
    cg.force_size(sz);
}

fn exe_settled(app: &mut App) {
   app.settle(); 
}

fn exe_component_add(a: &mut App, name: &str) {
    if let Some(c) = a.get_component(name) {
        a.with_compo(|co| {
            let cs = co.get_component_set();
            cs.add(c)
        });
    }
}

fn exe_set_stick(a: &mut App, name: &str) {
    if let Some(stick) = a.with_stick_manager(|sm| sm.get_stick(name)) {
        a.with_compo(|co| co.set_stick(&stick));
        a.with_stage(|s| {
            s.set_limit(&LEFT,0.);
            s.set_limit(&RIGHT,stick.length() as f64);
            s.set_wrapping(&stick.get_wrapping());
            
        });
        exe_pos_event(a,cdfraction(0.,0.),None);
    }
}

fn exe_set_state(a: &mut App, name: &str, on: bool) {
    a.with_state(|s| {
        s.set_atom_state(name,on);
    });
}

pub fn actions_run(cg: &mut App, evs: &Vec<Action>) {
    for ev in evs {
        let ev = ev.clone();
        match ev {
            Action::Pos(v,prop) => exe_pos_event(cg,v,prop),
            Action::PosRange(x_start,x_end,y) => exe_pos_range_event(cg,x_start,x_end,y),
            Action::Move(v) => exe_move_event(cg,v),
            Action::Zoom(z) => exe_zoom_event(cg,z,true),
            Action::ZoomTo(z) => exe_zoom_event(cg,z,false),
            Action::Resize(sz) => exe_resize(cg,sz),
            Action::AddComponent(name) => exe_component_add(cg,&name),
            Action::SetStick(name) => exe_set_stick(cg,&name),
            Action::SetState(name,on) => exe_set_state(cg,&name,on),
            Action::Settled => exe_settled(cg),
            Action::Noop => ()
        }
    }
}

pub fn startup_actions() -> Vec<Action> {
    vec! {
        Action::Pos(Dot(0_f64,0_f64),None),
    }
}
