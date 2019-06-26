use std::sync::{ Arc, Mutex };

use stdweb::unstable::TryInto;
use url::Url;

use controller::global::Global;
use controller::input::Action;
use debug::DEMO_SOURCES;

use dom::domutil;
use dom::event::{ EventListener, EventType, EventData, EventControl, Target };
use dom::AppEventData;

pub struct StartupEventListener {
    g: Global
}

impl StartupEventListener {
    pub fn new(g: &Global) -> StartupEventListener {
        StartupEventListener {
            g: g.clone()
        }
    }    
}

impl EventListener<()> for StartupEventListener {
    fn receive(&mut self, _el: &Target,  e: &EventData, _idx: &()) {
        match e {
            EventData::CustomEvent(_,cx,name,data) => {
                let aed = AppEventData::new(data);
                match name.as_ref() {
                    "bpane-activate" => {
                        let key = unwrap!(aed.get_simple_str("key",Some("only")));
                        let debug = unwrap!(aed.get_simple_bool("debug",Some(false)));
                        console!("Activate browser {} on {:?}",key,cx.target());
                        let config_url = aed.get_simple_str("config-url",None);
                        if config_url.is_none() {
                            console!("BROWSER APP REFUSING TO START UP! No config-url supplied");
                        }
                        let config_url = ok!(Url::parse(&unwrap!(config_url)));
                        self.g.trigger_app(&key,&unwrap!(cx.target().try_into()),debug,&config_url);
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }
}

pub fn register_startup_events(g: &Global) {
    let uel = StartupEventListener::new(g);
    let mut ec_start = EventControl::new(Box::new(uel),());
    ec_start.add_event(EventType::CustomEvent("bpane-activate".to_string()));
    ec_start.add_element(&domutil::query_select("body"),());
}

pub fn initial_actions() -> Vec<Action> {
    let mut out = Vec::<Action>::new();
    
    /* Default tracks */
    for name in &DEMO_SOURCES {
        console!("activating {}",name);
        out.push(Action::AddComponent(name.to_string()));
        out.push(Action::SetState(name.to_string(),true));
    }
    out
}
