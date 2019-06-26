use std::sync::{ Arc, Mutex };

use serde_json::Value as JSONValue;
use stdweb::web::{ Element, HtmlElement };

use controller::global::{ App, AppRunner };
use controller::input::{ actions_run, Action };
use dom::event::{ 
    EventListener, EventControl, EventType, EventData, 
    ICustomEvent, Target
};
use types::{ Move, Distance, Units };

fn custom_movement_event(dir: &str, unit: &str, v: &JSONValue) -> Action {
    if let JSONValue::Number(quant) = v {
        let quant = quant.as_f64().unwrap() as f64;
        let unit = match unit {
            "base"|"bases"|"bp" => Units::Bases,
            "pixel"|"pixels"|"px" => Units::Pixels,
            "screen"|"screens"|"sc" => Units::Screens,
            _ => { return Action::Noop; }
        };
        Action::Move(match dir {
            "left" => Move::Left(Distance(quant,unit)),
            "right" => Move::Right(Distance(quant,unit)),
            "up" => Move::Up(Distance(quant,unit)),
            "down" => Move::Down(Distance(quant,unit)),
            _ => { return Action::Noop; }
        })
    } else {
        Action::Noop
    }
}

fn custom_goto_event(v: &JSONValue) -> Action {
    if let JSONValue::String(ref v) = v {
        let parts : Vec<&str> = v.split("-").collect();
        if parts.len() == 2 {
            let start : Result<i64,_> = parts[0].parse();
            let end : Result<i64,_> = parts[1].parse();
            if start.is_ok() && end.is_ok() {
                return Action::PosRange(start.ok().unwrap() as f64,
                                 end.ok().unwrap() as f64,0.);
            }
        }
    }
    Action::Noop
}

fn custom_stick_event(v: &JSONValue) -> Action {
    if let JSONValue::String(v) = v {
        Action::SetStick(v.to_string())
    } else {
        Action::Noop
    }
}

fn custom_zoom_event(kind: &str, v: &JSONValue) -> Action {
    if let JSONValue::Number(quant) = v {
        let quant = quant.as_f64().unwrap();
        match kind {
            "by" => {
                Action::Zoom(quant)
            },
            _ => Action::Noop
        }
    } else {
        Action::Noop
    }
}

fn custom_state_event(v: &JSONValue, sv: bool) -> Action {
    if let JSONValue::String(track) = v {
        Action::SetState(track.to_string(),sv)
    } else {
        Action::Noop
    }
}

fn every<F>(v: &JSONValue, cb: F) -> Vec<Action> where F: Fn(&JSONValue) -> Action {
    if let JSONValue::Array(vv) = v {
        vv.iter().map(|x| cb(x)).collect()
    } else {
        vec! {cb(v) }
    }
}

fn custom_make_one_event_key(k: &String, v: &JSONValue) -> Vec<Action> {
    let parts : Vec<&str> = k.split("_").collect();
    match parts.len() {
        1 => return match parts[0] {
            "on" => every(v,|v| custom_state_event(v,true)),
            "off" => every(v,|v| custom_state_event(v,false)),
            "goto" => every(v,|v| custom_goto_event(v)),
            "stick" => every(v,|v| custom_stick_event(v)),
            _ => vec!{}
        },
        2 => return match parts[0] {
            "zoom" => every(v,|v| custom_zoom_event(parts[1],v)),
            _ => vec!{}
        },
        3 => return match parts[0] {
            "move" => every(v,|v| custom_movement_event(parts[1],parts[2],v)),
            _ => vec!{}
        },
        _ => vec!{}
    }
}

fn custom_make_events(j: &JSONValue) -> Vec<Action> {
    let mut out = Vec::<Action>::new();
    if let JSONValue::Object(map) = j {
        for (k,v) in map {
            out.append(&mut custom_make_one_event_key(k,v));
        }
    }
    console!("receive/A {}",j);
    out
}

pub fn run_direct_events(app: &mut App, j: &JSONValue) {
    let evs = custom_make_events(&j);
    console!("receive/B {:?}",evs);
    actions_run(app,&evs);
}

pub struct DirectEventListener {
    cg: Arc<Mutex<App>>,
}

impl DirectEventListener {
    pub fn new(cg: &Arc<Mutex<App>>) -> DirectEventListener {
        DirectEventListener { cg: cg.clone() }
    }        
}

impl EventListener<()> for DirectEventListener {    
    fn receive(&mut self, _el: &Target,  e: &EventData, _idx: &()) {
        if let EventData::CustomEvent(_,_,_,c) = e {
            run_direct_events(&mut self.cg.lock().unwrap(),
                              &c.details().unwrap());
        }
    }
}

pub fn register_direct_events(gc: &mut AppRunner, el: &HtmlElement) {
    let elel : Element = el.clone().into();
    let dlr = DirectEventListener::new(&gc.state());
    let mut ec = EventControl::new(Box::new(dlr),());
    ec.add_event(EventType::CustomEvent("bpane".to_string()));
    ec.add_element(&elel,());
    gc.add_control(Box::new(ec));
}
