#[cfg(not(deploy))]
pub mod closuresource;

mod sources;
mod stickmanager;

pub use self::sources::{ add_debug_sources, DEBUG_SOURCES, DEMO_SOURCES };
pub use self::stickmanager::{ add_debug_sticks };
