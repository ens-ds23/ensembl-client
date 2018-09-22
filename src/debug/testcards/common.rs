use std::sync::{ Mutex, Arc };
use debug::testcards::visual::testcard_visual;
use debug::testcards::button::testcard_button;
use debug::testcards::polar::testcard_polar;
use debug::testcards::text::testcard_text;
use types::{ CLeaf, cleaf };
use rand::{ Rng, seq };
use rand::distributions::Distribution;
use rand::distributions::range::Range;
use controller::Global;

pub fn testcard(g: Arc<Mutex<Global>>, name: &str) {
    debug!("global","starting testcard {}",name);
    match name {
        "draw" => testcard_visual(g,false),
        "onoff" => testcard_visual(g,true),
        "button" => testcard_button(g),
        "polar" => testcard_polar(g),
        "text" => testcard_text(g),
        _ => ()
    };
}

fn choose<R>(rng: &mut R, vals: &[&[&str]]) -> String
                    where R: Rng {
    let mut out = String::new();
    for val in vals {
        out += seq::sample_iter(rng,*val,1).unwrap()[0]
    }
    out
}

pub fn bio_daft<R>(rng: &mut R) -> String where R: Rng {
    let vals = [ "5'","3'","snp","ins","del",
                 "5'","3'","snp","ins","del",
                 "5'","3'","snp","ins","del",
                 "C","G","A","T" ];
    choose(rng,&[&vals[..]])
}

pub fn daft<R>(rng: &mut R) -> String where R: Rng {    
    let onset = [ "bl", "br", "ch", "cl", "cr", "dr", "fl",
                       "fr", "gh", "gl", "gr", "ph", "pl", "pr",
                       "qu", "sc", "sh", "sk", "sl", "sm", "sn", "sp",
                       "st", "sw", "th", "tr", "tw", "wh", "wr",
                       "sch", "scr", "shr", "spl", "spr", "squ",
                       "str", "thr", "b", "c", "d", "f", "g", "h", "j",
                       "k", "l", "m", "n", "p", "r", "s", "t", "u", "v",
                       "w", "x", "y", "z" ];
    let nuc = [ "ai", "au", "aw", "ay", "ea", "ee", "ei", "eu",
                    "ew", "ey", "ie", "oi", "oo", "ou", "ow", "oy",
                    "a", "e", "i", "o", "u" ];
    let coda = [  "ch", "ck", "gh", "ng", "ph", "sh", "sm", "sp",
                       "st", "th",  "nth", 
                       "b", "c", "d", "f", "g", "h", "j",
                       "k", "l", "m", "n", "p", "r", "s", "t", "u", "v",
                       "w", "x", "y", "z" ];
    let num_gen = Range::new(1,8);
    let mut out = String::new();
    let num = num_gen.sample(rng);
    for _i in 0..num {
        out += &choose(rng,&[&onset[..],&nuc[..],&coda[..]])[..];
        let sp: bool = rng.gen();
        if sp { out += " "; }
    }
    out
}

pub fn wiggly<R>(rng: &mut R, num: u32, origin: CLeaf, sep: f32, h: i32) 
                -> Vec<CLeaf> where R: Rng {
    let mut out = Vec::<CLeaf>::new();
    for i in 0..num {
        let v : i32 = rng.gen_range(0,h);
        out.push(origin + cleaf(i as f32*sep,v));
    }
    out
}

const MORSE_AL : &str = "abcdefghijklmnopqrstuvwxyz ";
const MORSE_DD : [&str;27] = [
    ".-", "-...", "-.-.", "-..", ".",
    "..-.", "--.", "....", "..", ".---",
    "-.-", ".-..", "--", "-.", "---",
    ".--.", "--.-", ".-.", "...", "-",
    "..-", "...-", ".--", "-..-", "-.--",
    "--..", " "
];

pub fn track_data(s: &str) -> Vec<f32> {
    let mut out = Vec::<f32>::new();
    for c in s.to_lowercase().chars() {
        if let Some(idx) = MORSE_AL.find(c) {
            for d in MORSE_DD[idx].chars() {
                out.push(match d {
                    '-' => 3.,
                    '.' => 1.,
                    _ => -9.
                });
                out.push(-1.);
            }
            out.push(-9.);
        }
    }
    out
}
