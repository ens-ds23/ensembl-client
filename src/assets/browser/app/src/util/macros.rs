// This common Rust macro is a shortcut to clone all its first list of
// arguments before evaluating the second argument
#[macro_export]
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

#[allow(unused_macros)]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[allow(unused_macros)]
macro_rules! hashmap_s {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key.to_string(), $val); )*
         map
    }}
}

#[allow(unused_macros)]
macro_rules! vec_s {
    ($( $val: expr ),*) => {{
        let mut vec = ::std::vec::Vec::new();
        $( vec.push($val.to_string()); )*
        vec
    }}
}

macro_rules! console_error {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        js! { console.error(@{s}); };
    }}
}

macro_rules! console_force {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        js! { console.log(@{s}); };
    }}
}

#[deprecated(note="use blackbox")]
macro_rules! console {
    ($($arg:tt)*) => {{
        if !cfg!(deploy) || cfg!(console) {
            console_force!($($arg)*);
        }
    }}
}

#[cfg(any(not(deploy),console))]
macro_rules! bb_time {
    ($stream:expr,$code:block) => {{
        let tmp_bb_enabled = ::data::blackbox::blackbox_is_enabled($stream);
        let tmp_bb_start = if tmp_bb_enabled {
             Some(::dom::domutil::browser_time())
        } else {
            None
        };
        let ret = (|| { $code })();
        if tmp_bb_enabled {
            let tmp_bb_end = ::dom::domutil::browser_time();
            ::data::blackbox::blackbox_elapsed($stream,tmp_bb_end-tmp_bb_start.unwrap());
        }
        ret
    }}
}

#[cfg(all(deploy,not(console)))]
macro_rules! bb_time {
    ($stream:expr,$code:block) => {{
        $code
    }}
}

#[cfg(any(not(deploy),console))]
macro_rules! bb_time_if {
    ($stream:expr,$code:block) => {{
        let tmp_bb_enabled = ::data::blackbox::blackbox_is_enabled($stream);
        let tmp_bb_start = if tmp_bb_enabled {
             Some(::dom::domutil::browser_time())
        } else {
            None
        };
        let ret = (|| { $code })();
        if tmp_bb_enabled && ret {
            let tmp_bb_end = ::dom::domutil::browser_time();
            ::data::blackbox::blackbox_elapsed($stream,tmp_bb_end-tmp_bb_start.unwrap());
        }
    }}
}

#[cfg(all(deploy,not(console)))]
macro_rules! bb_time_if {
    ($stream:expr,$code:block) => {{
        $code
    }}
}


#[cfg(any(not(deploy),console))]
macro_rules! bb_metronome {
    ($stream:expr) => {{
        if !cfg!(deploy) || cfg!(console) {
            if ::data::blackbox::blackbox_is_enabled($stream) {
                let tmp_bb = ::dom::domutil::browser_time();
                ::data::blackbox::blackbox_metronome($stream,tmp_bb);
            }
        }
    }}
}

#[cfg(all(deploy,not(console)))]
macro_rules! bb_metronome {
        ($stream:expr) => {{}}
}

#[cfg(any(not(deploy),console))]
macro_rules! bb_log {
    ($stream:expr,$($arg:tt)*) => {{
        if !cfg!(deploy) || cfg!(console) {
            if ::data::blackbox::blackbox_is_enabled($stream) {
                let s = format!($($arg)*);
                ::data::blackbox::blackbox_report($stream,&s);
            }
        }
    }}
}

#[cfg(all(deploy,not(console)))]
macro_rules! bb_log {
    ($stream:expr,$($arg:tt)*) => {{}}
}

#[cfg(any(not(deploy),console))]
macro_rules! bb_if_log {
    ($stream:expr,$code:block) => {{
        if !cfg!(deploy) || cfg!(console) {
            if ::data::blackbox::blackbox_is_enabled($stream) {
                $code
            }
        }
    }}
}

#[cfg(all(deploy,not(console)))]
macro_rules! bb_if_log {
    ($stream:expr,$code:block) => {{}}
}


#[allow(unused_macros)]
#[cfg(any(not(deploy),console))]
macro_rules! bb_stack {
    ($level:expr,$code:block) => {{
        ::data::blackbox::blackbox_push($level);
        let ret = (|| { $code })();
        ::data::blackbox::blackbox_pop();
        ret
    }}
}

#[allow(unused_macros)]
#[cfg(all(deploy,not(console)))]
macro_rules! bb_stack {
    ($level:expr,$code:block) => {{
        $code
    }}
}

macro_rules! unwrap {
    ($x: expr) => {{
        let s = format!("ENSEMBL ERROR LOCATION {}/{}/{}",file!(),line!(),column!());
        $x.expect(&s)
    }}
}

macro_rules! ok {
    ($x: expr) => {{
        let s = format!("ENSEMBL ERROR LOCATION {}/{}/{}",file!(),line!(),column!());
        let x = $x;
        if let Err(ref msg) = x {
            console_error!("OK Failed: {}",&msg);
        }
        x.expect(&s)
    }}
}
