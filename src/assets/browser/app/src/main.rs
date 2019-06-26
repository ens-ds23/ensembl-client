#![feature(non_ascii_idents)] // (help put the E into EMBL)
#![feature(linkage)]
#![feature(drain_filter)]

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;
extern crate rand;
extern crate itertools;
extern crate hsl;
extern crate url;
extern crate base64;

extern crate tánaiste;

#[macro_use]
mod util;

mod controller;
mod composit;
mod data;
mod debug;
mod dom;
mod drivers;
mod model;
mod tácode;
mod types;

use controller::global;
use util::build_summary;

fn main() {
    stdweb::initialize();
    global::setup_global();
    console_force!("{}",build_summary());
    stdweb::event_loop();
}
