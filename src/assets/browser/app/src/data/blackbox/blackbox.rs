use std::collections::{ HashMap, HashSet };
use std::sync::{ Arc, Mutex };

use serde_json::Value as SerdeValue;
use serde_json::Map as SerdeMap;
use stdweb::unstable::TryInto;

use dom::domutil::browser_time;
use util::get_instance_id;
use super::blackboxstate::BlackBoxState;
use super::blackboxdriver::BlackBoxDriver;

lazy_static! {
    static ref BLACKBOX: Mutex<BlackBoxState> = Mutex::new(BlackBoxState::new());
}

pub fn blackbox_tick(driver: &mut BlackBoxDriver) -> bool {
    let mut bb = BLACKBOX.lock().unwrap();
    let now = browser_time();
    driver.tick(&mut bb,now)
}

pub fn blackbox_is_enabled(stream: &str) -> bool {
    let mut bb = BLACKBOX.lock().unwrap();
    bb.is_enabled(stream)
}

pub fn blackbox_report(stream: &str, report: &str) {
    let mut bb = BLACKBOX.lock().unwrap();
    let now = browser_time();
    bb.report(stream,now,report);
}

pub fn blackbox_elapsed(stream: &str, elapsed: f64) {
    let mut bb = BLACKBOX.lock().unwrap();
    bb.elapsed(stream,elapsed);
}

pub fn blackbox_metronome(stream: &str, t: f64) {
    let mut bb = BLACKBOX.lock().unwrap();
    bb.metronome(stream,t);
}

pub fn blackbox_push(name: &str) {
    let mut bb = BLACKBOX.lock().unwrap();
    bb.push(name);
}

pub fn blackbox_pop() {
    let mut bb = BLACKBOX.lock().unwrap();
    bb.pop();
}
