use std::rc::Rc;
use std::sync::{ Arc, Mutex };
use serde_json::Value as JSONValue;
use stdweb::web::html_element::SelectElement;
use stdweb::traits::IEvent;
use stdweb::unstable::TryInto;
use stdweb::web::{ Element, IEventTarget, HtmlElement };
use stdweb::web::event::{ ChangeEvent, ClickEvent };

use controller::global::App;
use super::super::testcards::select_testcard;
use dom::{ Bling, NoBling };
use dom::domutil;
use dom::event::{
    EventListener, EventControl, EventType, EventData, Target,
    ICustomEvent
};
use util::truncate;

pub const DEBUGSTAGE : &str = r##"
<div class="bpane-container">
    <div class="bpane-canv">
        <h1>Debug Mode</h1>
    </div>
    <div class="bpane-right">
        <div class="console">
            <select class="testcard">
                <option value="">- testcards -</option>
                <option value="live">Live Testcard</option>
                <option value="button">Button Testcard</option>
                <option value="text">Text Testcard</option>
                <option value="text2">Text2 Testcard</option>
                <option value="ruler">Ruler Testcard</option>
                <option value="leaf">Leaf Testcard</option>
            </select>
            <select class="folder"></select>
            <button class="mark">mark!</button>
            <pre class="content console2"></pre>
        </div>
        <div class="events-out"></div>
        <div class="buttons"></div>
        <div class="managedcanvasholder"></div>
    </div>
</div>
"##;

pub const DEBUGSTAGE_CSS : &str = r##"
html, body {
    margin: 0px;
    padding: 0px;
    height: 100%;
    width: 100%;
    overflow: hidden;
}
.events-out {
    border: 1px solid blue;
    height: 64px;
    font-size: 10px;
    overflow-y: scroll;
}
.bpane-container {
    display: flex;
    height: 100%;
    width: 100%;
}
.bpane-container .bpane-right {
    width: 20%;
}

.bpane-container .console .content {
    height: 85%;
    overflow: scroll;
    border: 1px solid #ccc;
}

.bpane-container .managedcanvasholder {
    display: block;
    border: 2px solid red;
    display: inline-block;
    overflow: scroll;
    width: 100%;
}

.bpane-container, .bpane-container .bpane-canv canvas {
    height: 100%;
    width: 100%;
}

.bpane-container .bpane-canv {
    width: 80%;
    height: 100%;
}

.bpane-container .bpane-canv canvas {
    width: 100%;
    height: 100%;
}

#stage {
    height: 100%;
}

.bpane-container .console {
    height: 50%;
}
@import url('https://fonts.googleapis.com/css?family=Lato');
"##;

fn setup_debug_console(el: &HtmlElement) {
    let cons_el = domutil::query_selector_ok(&el.clone().into(),".console2","cannot find console");
    let mark_el = domutil::query_selector_ok(&el.clone().into(),".console .mark","cannot find console mark");
    mark_el.add_event_listener(enclose! { (cons_el) move |_e: ClickEvent| {
        domutil::send_custom_event(&cons_el,"mark",&json!({}));
    }});

}

fn setup_testcard_selector(a: &Arc<Mutex<App>>, el: &HtmlElement) {
    let a = a.clone();
    let sel_el = domutil::query_selector2(&el.clone().into(),".console .testcard").unwrap();
    sel_el.add_event_listener(enclose! { (a) move |e: ChangeEvent| {
        let mut a = a.lock().unwrap();
        let node : SelectElement = e.target().unwrap().try_into().ok().unwrap();
        if let Some(name) = node.value() {
            select_testcard(&mut a,&name);
        }
    }});
}

fn event_button(dii: &mut Vec<Box<DebugInteractor>>, name: &str, egg: Option<&str>, json: JSONValue) {
    let bi = ButtonDebugInteractor::new(name,egg,move |a| {
        let canv: HtmlElement = {
            a.lock().unwrap().get_canvas_element().clone().into()
        };
        domutil::send_custom_event(&canv.clone(),"bpane",&json.clone());
    });
    dii.push(bi);
}

pub fn create_interactors() -> Vec<Box<DebugInteractor>> {
    let mut dii = Vec::<Box<DebugInteractor>>::new();
    event_button(&mut dii,"shimmy",None,json!({ "move_left_px": 120, "move_down_screen": 0.25, "zoom_by": 0.1 }));
    event_button(&mut dii,"left",Some("h"),json!({ "move_left_px": 50 }));
    event_button(&mut dii,"right",Some("l"),json!({ "move_right_px": 50 }));
    event_button(&mut dii,"up",None,json!({ "move_up_px": 50 }));
    event_button(&mut dii,"down",None,json!({ "move_down_px": 50 }));
    event_button(&mut dii,"in",None,json!({ "zoom_by": 0.3 }));
    event_button(&mut dii,"out",None,json!({ "zoom_by": -0.3 }));
    event_button(&mut dii,"odd on",None,json!({ "on": "odd" }));
    event_button(&mut dii,"odd off",None,json!({ "off": "odd" }));
    event_button(&mut dii,"even on",None,json!({ "on": "even" }));
    event_button(&mut dii,"even off",None,json!({ "off": "even" }));
    event_button(&mut dii,"zero",None,json!({ "goto": "1000000-2000000" }));
    event_button(&mut dii,"ruler",None,json!({ "stick": "ruler" }));
    dii
}

pub trait DebugInteractor {
    fn name(&self) -> &str;
    fn render(&mut self, ar: &Arc<Mutex<App>>, el: &HtmlElement);
    fn key(&mut self, _app: &Arc<Mutex<App>>, _key: &str) {}
}

pub struct OutEventListener {
    target: HtmlElement
}

impl OutEventListener {
    pub fn new(target: &HtmlElement) -> OutEventListener {
        OutEventListener {
            target: target.clone()
        }
    }
}

const EVENT_LENGTH : usize = 1000;

impl EventListener<()> for OutEventListener {
    fn receive(&mut self, _el: &Target,  e: &EventData, _: &()) {
        if let EventData::CustomEvent(EventType::CustomEvent(_),_,s,d) = e {
            if s == "bpane-out" {
                let mut value = domutil::get_inner_html(&self.target.clone().into());
                value.push_str(&format!("{}\n",d.details().unwrap_or(json!{{}})));
                let too_long = value.len() - EVENT_LENGTH;
                if too_long > 0 {
                    value = truncate(&value,EVENT_LENGTH).to_string();
                }
                domutil::inner_html(&self.target.clone().into(),&value);
                domutil::scroll_to_bottom(&self.target.clone().into());
            }
        }
    }
}

fn setup_event_listener(el: &HtmlElement, target: &HtmlElement) {
    let mut li = EventControl::new(Box::new(OutEventListener::new(target)),());
    li.add_event(EventType::CustomEvent("bpane-out".to_string()));
    li.add_element(&el.clone().into(),());
}

pub struct ButtonEventListener<F> where F: Fn(&Arc<Mutex<App>>) {
    cb: Rc<F>
}
    
impl<F> ButtonEventListener<F>  where F: Fn(&Arc<Mutex<App>>) {
    pub fn new(cb: Rc<F>) -> ButtonEventListener<F> where F: Fn(&Arc<Mutex<App>>) {
        ButtonEventListener { cb: cb }
    }
}

impl<F> EventListener<Option<Arc<Mutex<App>>>> for ButtonEventListener<F> where F: Fn(&Arc<Mutex<App>>) {
    fn receive(&mut self, _el: &Target,  _e: &EventData, ar: &Option<Arc<Mutex<App>>>) {
        if let Some(ar) = ar {
            (self.cb)(ar);
        }
    }
}

pub struct ButtonDebugInteractor {
    name: String,
    ec: EventControl<Option<Arc<Mutex<App>>>>,
    cb: Rc<Fn(&Arc<Mutex<App>>)>
}

impl ButtonDebugInteractor {
    pub fn new<F>(name: &str, egg: Option<&str>, cb: F) -> Box<ButtonDebugInteractor> where F: Fn(&Arc<Mutex<App>>) + 'static {
        let cb = Rc::new(cb);
        let mut out = Box::new(ButtonDebugInteractor {
            ec: EventControl::new(Box::new(ButtonEventListener::new(cb.clone())),None),
            name: name.to_string(),
            cb
        });
        out.ec.add_event(EventType::MouseClickEvent);
        out
    }
}

impl DebugInteractor for ButtonDebugInteractor {
    fn name(&self) -> &str { &self.name }
    
    fn render(&mut self, ar: &Arc<Mutex<App>>, el: &HtmlElement) {
        if let Some(button_div) = domutil::query_selector2(&el,".buttons") {
            let button_el = domutil::append_element(&button_div,"button");
            domutil::text_content(&button_el,&self.name);
            self.ec.add_element(&button_el.clone().into(),Some(ar.clone()));
        }
    }    
}

pub struct DebugBling {
    mb: NoBling,
    dii: Vec<Box<DebugInteractor>>
}

impl DebugBling {
    pub fn new(dii: Vec<Box<DebugInteractor>>) -> DebugBling {
        DebugBling { 
            mb: NoBling::new(),
            dii
        }
    }
}

impl Bling for DebugBling {
    fn apply_bling(&self, el: &HtmlElement) -> HtmlElement {
        if let Some(old) = domutil::query_selector_new("#bpane-css") {
            domutil::remove(&old);
        }
        let css = domutil::append_element(&domutil::query_selector_ok_doc("head","no head element"),"style");
        domutil::inner_html(&css,DEBUGSTAGE_CSS);
        domutil::add_attr(&css,"id","bpane-css");
        domutil::inner_html(el,DEBUGSTAGE);
        domutil::query_selector(&el.clone().into(),".bpane-canv").clone().try_into().unwrap()
    }
    
    fn activate(&mut self, ar: &Arc<Mutex<App>>, el: &HtmlElement) {
        setup_testcard_selector(ar,el);
        setup_debug_console(el);
        if let Some(ref side) = domutil::query_selector2(&el.clone().into(),".events-out") {
            setup_event_listener(el,&side);
        }
        for di in &mut self.dii {
            di.render(&ar.clone(),&el.clone().into());
        }
    }   
    
    fn key(&mut self, app: &Arc<Mutex<App>>, key: &str) {
        for di in &mut self.dii {
            di.key(app,key);
        }
        self.mb.key(app,key);
    }
}
