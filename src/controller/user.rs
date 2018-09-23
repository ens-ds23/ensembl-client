use stdweb::unstable::TryInto;
use std::sync::{ Arc, Mutex };
use dom::domutil;
use std::rc::Rc;
use std::cell::RefCell;
use dom::event::{ EventListener, EventType, EventData, EventControl };
use stdweb::web::{ Element, HtmlElement, IHtmlElement };
use stdweb::traits::{ IEvent };
use dom::event;

use controller::{ Event, EventRunner };
use controller::physics::MousePhysics;
use controller::timers::Timers;
use controller::global::CanvasGlobalInst;

pub struct UserEventListener {
    canv_el: HtmlElement,
    runner: Rc<RefCell<EventRunner>>,
    mouse: Arc<Mutex<MousePhysics>>
}

impl UserEventListener {
    pub fn new(er: &Rc<RefCell<EventRunner>>,
               canv_el: &HtmlElement,
               mouse: &Arc<Mutex<MousePhysics>>) -> UserEventListener {
        UserEventListener {
            runner: er.clone(),
            mouse: mouse.clone(),
            canv_el: canv_el.clone()
        }
    }    
}

impl EventListener<()> for UserEventListener {    
    fn receive(&mut self, _el: &Element,  e: &EventData, _idx: &()) {
        let mut r = self.runner.borrow_mut();
        match e {
            EventData::MouseEvent(EventType::MouseWheelEvent,e) => {
                r.run(vec! { Event::Zoom(e.wheel_delta() as f32/1000.) })
            },
            EventData::MouseEvent(EventType::MouseDownEvent,e) => {
                self.canv_el.focus();
                domutil::clear_selection();
                e.stop_propagation();
                self.mouse.lock().unwrap().down(e.at());
            },
            EventData::MouseEvent(EventType::MouseMoveEvent,e) => { 
                self.mouse.lock().unwrap().move_to(e.at());
            },
            EventData::MouseEvent(EventType::MouseClickEvent,e) => {
                e.stop_propagation();
            }
            _ => ()
        };
    }
}

pub struct UserEventListenerBody {
    mouse: Arc<Mutex<MousePhysics>>
}

impl UserEventListenerBody {
    pub fn new(mouse: &Arc<Mutex<MousePhysics>>) -> UserEventListenerBody {
        UserEventListenerBody {
            mouse: mouse.clone()
        }
    }
}

impl EventListener<()> for UserEventListenerBody {    
    fn receive(&mut self, _el: &Element,  e: &EventData, _idx: &()) {
        if let EventData::MouseEvent(EventType::MouseUpEvent,_) = e {
            self.mouse.lock().unwrap().up();
        }
    }
}

pub fn register_user_events(
           gc: &mut CanvasGlobalInst,
           er: &Rc<RefCell<EventRunner>>, el: &Element) {
    event::disable_context_menu();
    let html_el: HtmlElement = el.clone().try_into().unwrap();
    let mp = Arc::new(Mutex::new(MousePhysics::new(&mut gc.timers)));
    let uel = UserEventListener::new(er,&html_el,&mp);
    let mut ec_canv = EventControl::new(Box::new(uel));
    ec_canv.add_event(EventType::MouseClickEvent);
    ec_canv.add_event(EventType::MouseDownEvent);
    ec_canv.add_event(EventType::MouseMoveEvent);
    ec_canv.add_event(EventType::MouseWheelEvent);        
    ec_canv.add_element(el.into(),());
    let uel_body = UserEventListenerBody::new(&mp);
    let mut ec_body = EventControl::new(Box::new(uel_body));
    ec_body.add_event(EventType::MouseUpEvent);
    ec_body.add_element(&domutil::query_select("body"),());        
    gc.cg.add_control(Box::new(ec_canv));
    gc.cg.add_control(Box::new(ec_body));
}
