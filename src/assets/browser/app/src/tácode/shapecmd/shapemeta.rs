/* When specifying a shape type, the metadata is structured as follows:
 * [type,colour-type,...]
 * 
 * Type is one of:
 *   0 = rectangle (two-anchor points)
 *   1 = stretchtangle (no anchor points)
 *   2 = hollow stretchtangle (no anchor points)
 *   3 = texture (one-anchor points)
 *   4 = wiggle (no anchor points)
 * 
 * Two anchor shapes are anchored at two places and can grow to suit
 * whether that's to the screen or genome zoom. Note that you may decide
 * not to configure a two-anchor shape to grow (such as a floating
 * rectangle), but they can.
 * 
 * colour-type is one of colour = 0; spot = 1. It is ignored for textures.
 * 
 * Two-anchor shapes:
 * There then follows two pairs representing the axis of each of the
 * sea-end of the anchors. [x1-axis,x2-axis,y1-axis,y2-axis]. Each has
 * one of three values 0 = genome/page; 1 = left/top; 2 = right/bottom.
 * Note that for any given axis if one axis is zero, both must be.
 * 
 * There then follow two pairs representing the ship-end of the anchors.
 * [x-type,x-delta,y-type,y-delta]. -types have one of three values,
 * 0 = left/top; 1 = middle; 2 = right/bottom. deltas are in pixels.
 * Then the (temporary) under integer is given: 0=normal, 
 * 1=fix under page, 2=fix under tape, 3=page under pin
 * 
 * 
 * For example [0,1,1,2,0,0,2,0,1,0,0] represents, entry by entry
 * 0. a rectangle 
 * 1. using spot colours
 * 1. Fixed at one end relative to the left of the screen
 * 2. Fixed at the other end relative to the right of the screen
 * 00. Free to scroll up and down
 * 2. Referenced by its right...
 * 0. ... extreme edge
 * 1. in the middle of that edge  ...
 * 0. ... precisely
 * 0. not in an under layer.
 * 
 * A more common value would be [0,1,0,0,0,0,0,0,0,0,0] representing an
 * object placed on the genome at some position referenced to its 
 * middle. Another might be [0,1,1,1,1,1,0,0,0,0,0] representing an
 * object relative top the top-left of the screen referenced to its own
 * top left, using spot colours not in an under layer.
 * 
 * For one anchor shapes the same structure is used but x2-axis and
 * y2-axis are replaced by x-scale and y-scale which are auxilliary
 * values used by
 * 
 * 1: texture to scale the texture
 * 
 * The stretchtangle and hollow stretchtangle are special. Only
 * the first two arguments are used.
 */

use drivers::webgl::{ TypeToShape };
use model::shape::{
    StretchWiggleTypeSpec, PinRectTypeSpec, StretchRectTypeSpec, 
    TextureTypeSpec
};
use types::AxisSense;

fn sea_option(meta: &Vec<f64>, idx: usize) -> Option<AxisSense> {
    match meta[idx] as i32 {
        0 => None,
        1 => Some(AxisSense::Max),
        2 => Some(AxisSense::Min),
        _ => None
    }
}

fn sea(meta: &Vec<f64>, idx: usize) -> Option<(AxisSense,AxisSense)> {
    let sea1 = sea_option(meta,idx);
    let sea2 = sea_option(meta,idx+1);
    if sea1.is_none() | sea2.is_none() { return None; }
    return Some((sea1.unwrap(),sea2.unwrap()));
}

fn ship(meta: &Vec<f64>, idx: usize) -> (Option<AxisSense>,i32) {
    (match meta[idx] as i32 {
        0 => Some(AxisSense::Max),
        1 => None,
        2 => Some(AxisSense::Min),
        _ => None
        
    },meta[idx+1] as i32)
}

fn make_rectangle(meta: &Vec<f64>) -> Option<Box<TypeToShape>> {
    Some(Box::new(PinRectTypeSpec {
        sea_x: sea(meta,2),
        sea_y: sea(meta,4),
        ship_x: ship(meta,6),
        ship_y: ship(meta,8),
        under: meta[10] as i32,
        spot: meta[1]!=0.
    }))
}

fn make_stretchtangle(meta: &Vec<f64>) -> Option<Box<TypeToShape>> {
    Some(Box::new(StretchRectTypeSpec {
        hollow: meta[0] == 2.,
        spot: meta[1]!=0.
    }))
}

fn make_texture(meta: &Vec<f64>) -> Option<Box<TypeToShape>> {
    Some(Box::new(TextureTypeSpec {
        sea_x: sea_option(meta,2),
        sea_y: sea_option(meta,4),
        ship_x: ship(meta,6),
        ship_y: ship(meta,8),
        under: meta[10] as i32,
        scale_x: meta[3] as f32,
        scale_y: meta[5] as f32
    }))
}

fn make_wiggle(_meta: &Vec<f64>) -> Option<Box<TypeToShape>> {
    Some(Box::new(StretchWiggleTypeSpec{}))
}

fn make_meta(meta: &Vec<f64>) -> Option<Box<TypeToShape>> {
    match meta[0] as i32 {
        0 => make_rectangle(meta),
        1|2 => make_stretchtangle(meta),
        3 => make_texture(meta),
        4 => make_wiggle(meta),
        _ => None
    }
}

pub fn build_meta(meta_iter: &mut Iterator<Item=&f64>) -> Option<Box<TypeToShape>> {
    let mut meta = Vec::<f64>::new();
    let first = *meta_iter.next().unwrap();
    meta.push(first);
    let len = if first == 1. || first == 2. || first == 4. { 1 } else { 10 };
    for _ in 0..len {
        meta.push(*meta_iter.next().unwrap());
    }
    let out = make_meta(&meta);
    out
}
