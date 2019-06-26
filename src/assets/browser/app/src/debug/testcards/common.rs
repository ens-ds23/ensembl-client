use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use rand::Rng;
use rand::RngCore;
use rand::prelude::SliceRandom;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use composit::Leaf;
use types::{ CLeaf, cleaf };
#[cfg(not(deploy))]
use types::Colour;

#[cfg(not(deploy))]
fn bytes_of_u64(v: u64) -> [u8;8] {
    [
        ((v>>56)&0xff) as u8,
        ((v>>48)&0xff) as u8,
        ((v>>40)&0xff) as u8,
        ((v>>32)&0xff) as u8,
        ((v>>24)&0xff) as u8,
        ((v>>16)&0xff) as u8,
        ((v>> 8)&0xff) as u8,
        ((v    )&0xff) as u8
    ]
}

pub fn prop(leaf: &Leaf, pos: i32) -> f32 {
    let mul = leaf.total_bp();
    let start_leaf = (leaf.get_index() as f64 * mul) as f64;
    ((pos as f64-start_leaf)/mul) as f32
}

const RNG_BLOCK_SIZE : i32 = 1000000;

#[cfg(not(deploy))]
pub fn rng_pos(kind: [u8;8], start: i32, end: i32, sep: i32, size: i32) -> Vec<[i32;2]> {
    let mut out = Vec::<[i32;2]>::new();
    let start_block = (start as f32/RNG_BLOCK_SIZE as f32).floor() as i32;
    let end_block = (end as f32/RNG_BLOCK_SIZE as f32).ceil() as i32;
    for block in start_block..(end_block+1) {
        let block_start = block * RNG_BLOCK_SIZE;
        let seed = [kind[0],kind[1],kind[2],kind[3],
                    kind[4],kind[5],kind[6],kind[7],
                    0,0,0,0,
                    ((block>>24)&0xff) as u8,
                    ((block>>16)&0xff) as u8,
                    ((block>> 8)&0xff) as u8,
                    ((block    )&0xff) as u8];
        let mut rng = SmallRng::from_seed(seed);
        let mut min_start = 0;
        while min_start < RNG_BLOCK_SIZE {
            let rnd_start = min_start + rng.gen_range(sep/2,sep);
            let rnd_end = rng.gen_range(0,size) + rnd_start;
            let obj_start = rnd_start + block_start;
            let obj_end = rnd_end + block_start;
            if obj_end >= start && obj_start <= end {
                out.push([obj_start,obj_end]);
            }
            min_start = rnd_end + rng.gen_range(size/2,size);
        }
    }
    out.sort();
    out
}

#[cfg(not(deploy))]
pub fn rng_seq(kind: [u8;8], start: i32, end: i32) -> String {
    let mut out = String::new();
    let start_block = (start as f32/RNG_BLOCK_SIZE as f32).floor() as i32;
    let end_block = (end as f32/RNG_BLOCK_SIZE as f32).ceil() as i32;
    for block in start_block..(end_block+1) {
        let mut block_start = block * RNG_BLOCK_SIZE;
        let seed = [kind[0],kind[1],kind[2],kind[3],
                    kind[4],kind[5],kind[6],kind[7],
                    0,0,0,0,
                    ((block>>24)&0xff) as u8,
                    ((block>>16)&0xff) as u8,
                    ((block>> 8)&0xff) as u8,
                    ((block    )&0xff) as u8];
        let mut rng = SmallRng::from_seed(seed);
        while block_start < start {
            rng.next_u32();
            block_start += 1;
        }        
        for _ in 0..(end-start) {
            let v : usize = (rng.next_u32()%4) as usize;
            out.push_str(&["C","G","A","T"][v]);
        }
    }
    out
}

#[cfg(not(deploy))]
fn start_hash(kind: [u8;8], start: &[i32;2]) -> u64 {
    let mut h = DefaultHasher::new();
    h.write(&kind);
    h.write_i32(start[0]);
    h.write_i32(start[1]);
    h.finish()
}

#[cfg(not(deploy))]
pub fn start_rng(kind: [u8;8], start: &[i32;2]) -> SmallRng {
    let b = bytes_of_u64(start_hash(kind,start));
    let seed = [b[0],b[1],b[2],b[3],b[4],b[5],b[6],b[7],
                kind[0],kind[1],kind[2],kind[3],
                kind[4],kind[5],kind[6],kind[7]];
    SmallRng::from_seed(seed)
}

#[cfg(not(deploy))]
pub fn rng_colour(kind: [u8;8], start: &[i32;2]) -> Colour {
    let b = bytes_of_u64(start_hash(kind,start));
    Colour(b[0] as u32,b[1] as u32,b[2] as u32)
}

#[cfg(not(deploy))]
const MANY : u64 = 1000000000;
#[cfg(not(deploy))]
pub fn rng_prob(kind: [u8;8], start: &[i32;2], prob: f32) -> bool {
    let b = start_hash(kind,start);
    (b%MANY) < (prob * MANY as f32) as u64
}

#[cfg(not(deploy))]
pub fn rng_choose(kind: [u8;8], start: &[i32;2], vals: &[&[&str]]) -> String {
    let mut rng = start_rng(kind,start);
    let mut out = String::new();
    for val in vals {
        out += val[..].choose(&mut rng).unwrap();
    }
    out
}

#[cfg(not(deploy))]
pub fn rng_subdivide(kind: [u8;8], extent: &[i32;2], parts: u32) -> Vec<[i32;2]> {
    let mut rng = start_rng(kind,extent);
    let mut breaks = Vec::<i32>::new();
    for _ in 0..parts {
        breaks.push(rng.gen_range(extent[0],extent[1]));
    }
    breaks.push(extent[0]);
    breaks.push(extent[1]);
    breaks.sort();
    let mut out = Vec::<[i32;2]>::new();
    for i in 0..breaks.len()-1 {
        out.push([breaks[i],breaks[i+1]]);
    }
    out
}

fn choose<R>(rng: &mut R, vals: &[&[&str]]) -> String
                    where R: Rng {
    let mut out = String::new();
    for val in vals {
        out += val[..].choose(rng).unwrap();
    }
    out
}

#[cfg(not(deploy))]
pub fn bio_mark(kind: [u8;8], start: &[i32;2]) -> String {
    let vals = [ "5'","3'","snp","ins","del",
                 "5'","3'","snp","ins","del",
                 "5'","3'","snp","ins","del",
                 "C","G","A","T" ];
    rng_choose(kind,start,&[&vals[..]])
}

#[cfg(not(deploy))]
pub fn rng_tracks(kind: [u8;8], num: i32) -> Vec<String> {    
    let onset = [
        "bl", "br", "ch", "cl", "cr", "dr", "fl", "fr", "gh", "gl", 
        "gr", "ph", "pl", "pr", "qu", "sc", "sh", "sk", "sl", "sm", 
        "sn", "sp", "st", "sw", "th", "tr", "tw", "wh", "wr", "sch", 
        "scr", "shr", "spl", "spr", "squ", "str", "thr", "b", "c", "d",
        "f", "g", "h", "j", "k", "l", "m", "n", "p", "r", "s", "t", 
        "u", "v", "w", "x", "y", "z"
    ];
    let nuc = [
        "ai", "au", "aw", "ay", "ea", "ee", "ei", "eu", "ew", "ey",
        "ie", "oi", "oo", "ou", "ow", "oy", "a", "e", "i", "o", "u"
    ];
    let coda = [
        "ch", "ck", "gh", "ng", "ph", "sh", "sm", "sp", "st", "th",
        "nth", "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", 
        "p", "r", "s", "t", "u", "v", "w", "x", "y", "z"
    ];
    let mut out = Vec::<String>::new();
    let mut rng = start_rng(kind,&[0,0]);
    for _ in 0..num {
        let mut s = String::new();
        let num : i32 = rng.gen_range(1,8);
        for _i in 0..num {
            s += &choose(&mut rng,&[&onset[..],&nuc[..],&coda[..]])[..];
            let sp: bool = rng.gen();
            if sp { s += " "; }
        }
        out.push(s);
    }
    out
}

fn scale_wiggle(in_: &Vec<CLeaf>, start: i32, end: i32) -> Vec<CLeaf> {
    let size = end-start;
    in_.iter().map(|x| {
        cleaf((x.0-start as f32)/size as f32,x.1)
    }).collect()
}

pub fn delta(fun: &Vec<f32>, pos: f32, scale: f32) -> f32 {
    let a = ((pos/scale)%(fun.len() as f32-1.)).abs() as usize;
    let d = ((pos%scale)/scale).abs();
    let p = d*d;
    fun[a] * (1.-p) + fun[a+1] * p
}

pub fn wiggle_func(fun: &Vec<f32>, pos: f32) -> f32 {
    let slow = delta(fun,pos,10000.);
    let fast = delta(fun,pos,100.);
    let out = (pos/1000.).cos()*0.1+slow*0.7+fast*0.2;
    out
}

pub fn wiggly(num: i32, start: i32, end: i32, offset: i32, h: i32) -> Vec<CLeaf> {
    let mut rng = SmallRng::from_seed([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    let mut fun = Vec::<f32>::new();
    for _ in 0..100 {
        fun.push(rng.gen_range(0.,1.));
    }
    let mut out = Vec::<CLeaf>::new();
    let mut x_pos = start as f32;
    let x_inc = (end-start) as f32/(num as f32-1.);
    for _ in 0..num {
        let v : i32 = (wiggle_func(&fun,x_pos)*h as f32) as i32;
        out.push(cleaf(x_pos,offset+v));
        x_pos += x_inc;
    }
    scale_wiggle(&out,start,end)
}
