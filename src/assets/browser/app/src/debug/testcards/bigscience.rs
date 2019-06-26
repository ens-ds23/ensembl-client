#![allow(unused)]

use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use std::rc::Rc;

use rand::distributions::Distribution;
use rand::Rng;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use composit::{
    StateFixed, StateAtom, Leaf, Stick
};

use debug::support::closuresource::{ ClosureSource, closure_add, closure_add_opt, closure_done };
use debug::testcards::common::{
    wiggly, rng_prob, rng_pos, rng_colour, start_rng,
    rng_subdivide, bio_mark, rng_tracks, prop
};

use model::shape::{
    ColourSpec, MathsShape, stretch_wiggle,
    PinRectTypeSpec, StretchRectTypeSpec, 
    tape_mathsshape,
    pin_mathsshape,
    fix_mathsshape, page_mathsshape,
    text_texture,   bitmap_texture,
    MarkSpec, DrawingSpec,
    TextureTypeSpec,
};
use model::train::PartyResponses;

use drivers::webgl::{ Facade, ShapeShortInstanceData, stretch_texture };

use controller::global::App;

use types::{
    Colour, cleaf, cpixel, area, cedge, AxisSense,
    TOPLEFT, BOTTOMLEFT, TOPRIGHT, BOTTOMRIGHT, area_size,
    A_MIDDLE, A_BOTTOMRIGHT, A_BOTTOMLEFT, A_TOPLEFT, A_TOPRIGHT, A_TOP,
    A_RIGHT, A_LEFT
};

use drivers::webgl::{
    collage, FCFont,
    mark_rectangle, FontVariety,
};

const SW : i32 = 1000;
const SH : i32 = 1000;

fn battenberg() -> DrawingSpec {
    bitmap_texture(vec! { 0,0,255,255,
                          255,0,0,255,
                          0,255,0,255,
                          255,255,0,255 },cpixel(2,2),false,None)
}

fn measure(lc: &mut PartyResponses, leaf: &Leaf, cs: &ColourSpec, cs2: &ColourSpec) {
    for x in -10..10 {
        let prts = PinRectTypeSpec {
            sea_x: None,
            sea_y: Some((AxisSense::Max,AxisSense::Max)),
            ship_x: (Some(AxisSense::Min),0),
            ship_y: (Some(AxisSense::Min),0),
            under: 0,
            spot: false
        };
        closure_add_opt(lc,&prts.new_short_shape(&ShapeShortInstanceData {
            pos_x: x as f32/10.,
            pos_y: 0,
            aux_x: 20.,
            aux_y: 20,
            facade: Facade::Colour(Colour(255,100,50))
        }));
        closure_add(lc,&tape_mathsshape(
            &cleaf(x as f32/10.+25.,0).y_edge(AxisSense::Max),
            A_TOP,
            10., None, MathsShape::Polygon(5,0.05),
            cs2));
        
        let tts = TextureTypeSpec {
            sea_x: None,
            sea_y: Some(AxisSense::Max),
            ship_x: (None,0),
            ship_y: (Some(AxisSense::Max),0),
            under: 0,
            scale_x: 10., scale_y: 10.
        };
        closure_add_opt(lc,&tts.new_short_shape(&ShapeShortInstanceData {
            pos_x: x as f32/10.+0.05,
            pos_y: 0,
            aux_x: 0.,
            aux_y: 0,
            facade: Facade::Drawing(battenberg())
        }));
        closure_add(lc,&tape_mathsshape(
            &cleaf(x as f32/10.+0.075,0).y_edge(AxisSense::Max),
            A_TOP,
            10., Some(1.), MathsShape::Circle,
            cs));
    }
}

fn make_rng(seed: i32) -> SmallRng {
    let seed = 12345678;
    let s = seed as u8;
    let t = (seed/256) as u8;
    SmallRng::from_seed([s,s,s,s,s,s,s,s,t,t,t,t,t,t,t,t])
}


fn source_odd() -> ClosureSource {
    let seed = 12345678;
    
    let pal = Palette::new();
    ClosureSource::new(0.,enclose! { (pal) move |lc,leaf| {
        let mul = leaf.total_bp();
        let start_leaf = (leaf.get_index() as f64 * mul).floor() as i32;
        let end_leaf = ((leaf.get_index()+1) as f64 * mul).ceil() as i32;
        let mut rng = make_rng(seed);
        for yidx in 0..20 {
            let wiggle = 200000;
            let poly_rng = rng_pos([yidx as u8,0,0,0,0,0,0,8],start_leaf-wiggle,end_leaf,400000,10000);
            let y = yidx * 60;
            if yidx == pal.middle - 5 {
                if start_leaf < 100000 && end_leaf > 0 {
                    let start_prop = prop(leaf,0);
                    for i in 1..10 {
                        closure_add(lc,&pin_mathsshape(&cleaf(start_prop+10000.*(i as f32)/mul as f32,y+20),
                           A_MIDDLE,
                           10. * i as f32,None,MathsShape::Circle,
                           &pal.green));
                    }
                }
            }
            if yidx == pal.middle {
                for p in poly_rng.iter() {
                    let start_prop = prop(leaf,p[0]);
                    for i in 3..8 {
                        closure_add(lc,&pin_mathsshape(&cleaf(start_prop+10000.*(i as f32)/mul as f32,y+20),
                                       A_TOP,
                                       10., None, MathsShape::Polygon(i,0.2*i as f32),
                                       &pal.red));
                    }
                }
                if start_leaf < 100000 && end_leaf > 0 {
                    let start_prop = prop(leaf,0);
                    let srts = StretchRectTypeSpec { spot: true, hollow: false };
                    closure_add_opt(lc,&srts.new_short_shape(&ShapeShortInstanceData {
                        pos_x: start_prop,
                        pos_y: y-20,
                        aux_x: 10000./mul as f32,
                        aux_y: 5,
                        facade: Facade::Colour(Colour(255,100,50))
                    }));
                    closure_add(lc,&pin_mathsshape(
                        &cleaf(start_prop,y-15),
                        A_RIGHT,
                        5.,None,
                        MathsShape::Polygon(3,0.),
                        &pal.red));
                }
            }
            if yidx == pal.middle+2 || yidx == pal.middle+4 {
                let wiggle = wiggly(500,start_leaf,end_leaf,y-5,20);
                closure_add(lc,&stretch_wiggle(wiggle,2,&pal.green_col));
            }
            if yidx == pal.middle + 1 {
                for p in poly_rng.iter() {
                    let start_prop = prop(leaf,p[0]);
                    for i in (3..7).step_by(2) {
                        closure_add(lc,&pin_mathsshape(&cleaf(start_prop+10000.*(i as f32+0.5)/mul as f32,y+20),
                                       A_TOP,
                                       10., Some(2.), MathsShape::Polygon(i,0.2*i as f32),
                                       &pal.red));
                        let colour = Colour(0,128,255);
                        closure_add(lc,&pin_mathsshape(&cleaf(start_prop+10000.*(i as f32)/mul as f32,y+20),
                                       A_TOP,
                                       10., Some(2.), MathsShape::Polygon(i,0.2*i as f32),
                                       &ColourSpec::Colour(colour)));
                    }
                }
            }
        }
        closure_done(lc);
    }})
}

fn source_even() -> ClosureSource {
    let seed = 12345678;
    
    let pal = Palette::new();
    ClosureSource::new(0.,enclose! { (pal) move |lc,leaf| {
        let mut rng = make_rng(seed);
        let mul = leaf.total_bp();
        let start_leaf = (leaf.get_index() as f64 * mul).floor() as i32;
        let end_leaf = ((leaf.get_index()+1) as f64 * mul).ceil() as i32;
        for yidx in 0..20 {
            let wiggle = 200000;
            let poly_rng = rng_pos([yidx as u8,0,0,0,0,0,0,8],start_leaf-wiggle,end_leaf,400000,10000);
            let y = yidx * 60;
            if yidx == pal.middle - 5 {
                if start_leaf < 100000 && end_leaf > 0 {
                    let start_prop = prop(leaf,0);
                    for i in 1..10 {
                        let colour = Colour(255,0,128);
                        closure_add(lc,&pin_mathsshape(&cleaf(start_prop+10000.*(i as f32)/mul as f32,y+20),
                                       A_MIDDLE,
                                       10. * i as f32,Some(2.),MathsShape::Circle,
                                       &ColourSpec::Colour(colour)));
                    }
                }
            }
            if yidx == pal.middle {
                for p in poly_rng.iter() {
                    let start_prop = prop(leaf,p[0]);
                    for i in 3..8 {
                        let colour = Colour(0,128,255);
                        closure_add(lc,&pin_mathsshape(&cleaf(start_prop+10000.*(i as f32+8.)/mul as f32,y+20),
                           A_TOP,
                           10., None, MathsShape::Polygon(i,0.2*i as f32),
                           &ColourSpec::Colour(colour)));

                    }
                }
                if start_leaf < 100000 && end_leaf > 0 {
                    let start_prop = prop(leaf,0);      
                    let srts = StretchRectTypeSpec { spot: true, hollow: false };
                    closure_add_opt(lc,&srts.new_short_shape(&ShapeShortInstanceData {
                        pos_x: start_prop,
                        pos_y: y-15,
                        aux_x: 10000./mul as f32,
                        aux_y: 5,
                        facade: Facade::Colour(Colour(50,255,150))
                    }));
                    closure_add(lc,&pin_mathsshape(
                        &cleaf(10000./mul as f32,y-15),
                        A_LEFT,
                        5.,None,
                        MathsShape::Polygon(3,0.5),
                        &pal.green));
                }
            }
            if yidx == pal.middle +1 {
                for p in poly_rng.iter() {
                    let start_prop = prop(leaf,p[0]);
                    for i in (4..8).step_by(2) {
                        closure_add(lc,&pin_mathsshape(&cleaf(start_prop+10000.*(i as f32+0.5)/mul as f32,y+20),
                                       A_TOP,
                                       10., Some(2.), MathsShape::Polygon(i,0.2*i as f32),
                                       &pal.red));
                        let colour = Colour(0,128,255);
                        closure_add(lc,&pin_mathsshape(&cleaf(start_prop+10000.*(i as f32)/mul as f32,y+20),
                                       A_TOP,
                                       10., Some(2.), MathsShape::Polygon(i,0.2*i as f32),
                                       &ColourSpec::Colour(colour)));                    
                    }
                }
            }
        }
        closure_done(lc);
    }})
}

fn tinsel() -> Vec<u8> {
    vec! {
        0,0,255,255,   255,0,0,255,  0,255,0,255,  255,255,0,255,
        255,255,0,255, 0,255,0,255,  255,0,0,255,  0,0,255,255,
    }
}

pub fn bs_source_sub(even: bool) -> ClosureSource {
    if even { source_even() } else { source_odd() }
}

const TINSEL_LENGTH : i32 = 100000;
const WALL_LENGTH : i32 = 1000000;

fn round_down(num: i32, denom: i32) -> i32 {
    ((num as f32/denom as f32).floor() * denom as f32) as i32
}

fn bs_collage() -> DrawingSpec {
    let mut rng = start_rng([0,0,0,0,0,0,0,0],&[0,0]);
    let mut parts = Vec::<MarkSpec>::new();
    for row in 0..8 {
        let mut off = 0;
        for _pos in 0..100 {
            let size = rng.gen_range(1,14);
            let gap = rng.gen_range(1,10);
            if off + gap + size > 1000 { continue }
            off += gap;
            parts.push(mark_rectangle(
                &area_size(cpixel(off,row*5),cpixel(size,4)),
                &Colour(255,200,100)));
            if rng.gen_range(0,2) == 1 {
                parts.push(mark_rectangle(
                    &area_size(cpixel(off,row*5),cpixel(1,4)),
                    &Colour(200,0,0)));
            }
            if rng.gen_range(0,2) == 1 {
                parts.push(mark_rectangle(
                    &area_size(cpixel(off+size-1,row*5),cpixel(1,4)),
                    &Colour(0,0,200)));
            }
            off += size;
        }
    }
    collage(parts,cpixel(1000,40))
}

pub fn bs_source_main() -> ClosureSource {
    let seed = 12345678;
    
    let pal = Palette::new();
    ClosureSource::new(0.,enclose! { (pal) move |ref mut lc,leaf| {
        let mul = leaf.total_bp();
        let start_leaf = (leaf.get_index() as f64 * mul).floor() as i32;
        let end_leaf = ((leaf.get_index()+1) as f64 * mul).ceil() as i32;
        measure(lc,&leaf,&pal.red,&pal.green);
        let tracks = rng_tracks([0,0,0,0,0,0,0,10],20);
        for yidx in 0..20 {
            let genestart_rng = rng_pos([yidx as u8,0,0,0,0,0,0,0],start_leaf,end_leaf,10000,10000);
            let bberg_rng = rng_pos([yidx as u8,0,0,0,0,0,0,6],start_leaf,end_leaf,100000,10000);
            let y = yidx * 60;
            let tx = text_texture(&tracks[yidx as usize],&pal.fc_font,&pal.col,&Colour(255,255,255));
            
            let tts = TextureTypeSpec {
                sea_x: Some(AxisSense::Max),
                sea_y: None,
                ship_x: (Some(AxisSense::Max),0),
                ship_y: (Some(AxisSense::Max),0),
                under: 0,
                scale_x: 1., scale_y: 1.
            };
            closure_add_opt(lc,&tts.new_short_shape(&ShapeShortInstanceData {
                pos_x: 12 as f32,
                pos_y: y+18,
                aux_x: 0.,
                aux_y: 0,
                facade: Facade::Drawing(tx)
            }));
            closure_add(lc,&page_mathsshape(
                                &cpixel(0,y+18).x_edge(AxisSense::Max),
                                A_LEFT,
                                5.,None,MathsShape::Polygon(3,0.),
                                &pal.green));            
            if yidx == pal.middle+3 {
                if start_leaf < 100000 && end_leaf > 0 {
                    
                    let prts = PinRectTypeSpec {
                        sea_x: None,
                        sea_y: None,
                        ship_x: (Some(AxisSense::Min),0),
                        ship_y: (None,0),
                        under: 0,
                        spot: false
                    };
                    closure_add_opt(lc,&prts.new_short_shape(&ShapeShortInstanceData {
                        pos_x: prop(leaf,0),
                        pos_y: y-10,
                        aux_x: 20.,
                        aux_y: 20,
                        facade: Facade::Colour(Colour(128,0,0))
                    }));
                }
            }
            if yidx == pal.middle {
                let tx = bitmap_texture(tinsel(),
                                        cpixel(tinsel().len() as i32/4,1),true,None);
                let mut tinsel_start = round_down(start_leaf,TINSEL_LENGTH);
                while tinsel_start < end_leaf {
                    let pos_start = prop(leaf,tinsel_start);
                    let pos_end = prop(leaf,tinsel_start+TINSEL_LENGTH);
                    closure_add(lc,&stretch_texture(&tx,
                        &area(cleaf(pos_start,y-5),cleaf(pos_end,y+5))));
                    tinsel_start += TINSEL_LENGTH;
                }
                for p in bberg_rng.iter() {
                    let tx = bitmap_texture(
                                        vec! { 0,0,255,255,
                                                 255,0,0,255,
                                                 0,255,0,255,
                                                 255,255,0,255 },cpixel(2,2),false,None);
                    let start_prop = prop(leaf,p[0]);
                    let tts = TextureTypeSpec {
                        sea_x: None,
                        sea_y: None,
                        ship_x: (None,0),
                        ship_y: (None,0),
                        under: 0,
                        scale_x: 10., scale_y: 10.
                    };
                    closure_add_opt(lc,&tts.new_short_shape(&ShapeShortInstanceData {
                        pos_x: start_prop,
                        pos_y: y-25,
                        aux_x: 0.,
                        aux_y: 0,
                        facade: Facade::Drawing(tx),
                    }));
                }
            } else if yidx == pal.middle-2 {
                let tx = bs_collage();
                let mut wall_start = round_down(start_leaf,WALL_LENGTH);
                while wall_start < end_leaf {
                    let pos_start = prop(leaf,wall_start);
                    let pos_end = prop(leaf,wall_start+WALL_LENGTH);
                    closure_add(lc,&stretch_texture(&tx,&area(
                        cleaf(pos_start,y-5),cleaf(pos_end,y+45))
                    ));
                    wall_start += WALL_LENGTH;
                }
            } else if yidx == pal.middle+2 || yidx == pal.middle+4 {
                // no-op, wiggles
            } else {
                for (i,start) in genestart_rng.iter().enumerate() {
                    let colour = rng_colour([yidx as u8,0,0,0,0,0,0,1],start);
                    let mut hh = 0;
                    for (j,p) in rng_subdivide([yidx as u8,0,0,0,0,0,0,2],start,6).iter().enumerate() {
                        let h = hh+1;
                        hh = 5 -hh;
                        let start_prop = prop(leaf,p[0]);
                        let end_prop = prop(leaf,p[1]);
                        let srts = StretchRectTypeSpec { spot: false, hollow: false };
                        closure_add_opt(lc,&srts.new_short_shape(&ShapeShortInstanceData {
                            pos_x: start_prop,
                            pos_y: y-h,
                            aux_x: end_prop-start_prop,
                            aux_y: 2*h,
                            facade: Facade::Colour(colour)
                        }));
                        if rng_prob([yidx as u8,j as u8,0,0,0,0,0,5],start,0.2) {
                            let tri_col = rng_colour([yidx as u8,i as u8,j as u8,0,0,0,0,3],start);
                            closure_add(lc,&pin_mathsshape(
                                            &cleaf(start_prop,y+5),
                                            A_TOP,
                                            8.,None,
                                            MathsShape::Polygon(3,0.75),
                                            &ColourSpec::Colour(tri_col)));
                        }
                        if rng_prob([yidx as u8,j as u8,0,0,0,0,0,4],start,0.1) {
                            let val = bio_mark([yidx as u8,j as u8,0,0,0,0,0,7],start);
                            let tx = text_texture(&val,&pal.fc_font,&pal.col,&Colour(255,255,255));
                            let tts = TextureTypeSpec {
                                sea_x: None,
                                sea_y: None,
                                ship_x: (None,0),
                                ship_y: (None,0),
                                under: 0,
                                scale_x: 1., scale_y: 1.
                            };
                            closure_add_opt(lc,&tts.new_short_shape(&ShapeShortInstanceData {
                                pos_x: start_prop,
                                pos_y: y-12,
                                aux_x: 0.,
                                aux_y: 0,
                                facade: Facade::Drawing(tx)
                            }));
                        }
                    }
                }
            }
        }
        
        let prts = PinRectTypeSpec {
            sea_x: Some((AxisSense::Max,AxisSense::Max)),
            sea_y: Some((AxisSense::Max,AxisSense::Max)),
            ship_x: (Some(AxisSense::Min),0),
            ship_y: (Some(AxisSense::Min),0),
            under: 0,
            spot: true
        };

        closure_add_opt(lc,&prts.new_short_shape(&ShapeShortInstanceData {
            pos_x: SW as f32/2.,
            pos_y: 0,
            aux_x: 1.,
            aux_y: SH,
            facade: Facade::Colour(Colour(0,0,0))
        }));
        closure_add_opt(lc,&prts.new_short_shape(&ShapeShortInstanceData {
            pos_x: SW as f32/2.+5.,
            pos_y: 0,
            aux_x: 3.,
            aux_y: SH,
            facade: Facade::Colour(Colour(255,100,50))
        }));

        let tx = bitmap_texture(vec! { 0,0,255,255,
                                     255,0,0,255,
                                     0,255,0,255,
                                     255,255,0,255 },cpixel(1,4),false,None);
        let tts = TextureTypeSpec {
            sea_x: Some(AxisSense::Max),
            sea_y: Some(AxisSense::Max),
            ship_x: (Some(AxisSense::Max),0),
            ship_y: (Some(AxisSense::Max),0),
            under: 0,
            scale_x: 1., scale_y: SH as f32
        };
        closure_add_opt(lc,&tts.new_short_shape(&ShapeShortInstanceData {
            pos_x: SW as f32/2.-5.,
            pos_y: 0,
            aux_x: 0.,
            aux_y: 0,
            facade: Facade::Drawing(tx)
        }));
        for h in &[AxisSense::Min,AxisSense::Max] {
            for v in &[AxisSense::Min,AxisSense::Max] {
                let tts = TextureTypeSpec {
                    sea_x: Some(*h),
                    sea_y: Some(*v),
                    ship_x: (Some(*h),0),
                    ship_y: (Some(*v),0),
                    under: 0,
                    scale_x: 10., scale_y: 10.
                };
                closure_add_opt(lc,&tts.new_short_shape(&ShapeShortInstanceData {
                    pos_x: 0.,
                    pos_y: 0,
                    aux_x: 0.,
                    aux_y: 0,
                    facade: Facade::Drawing(battenberg())
                }));
            }
        }
        closure_add(lc,&fix_mathsshape(&cedge(TOPLEFT,cpixel(30,30)),
                                   A_TOPLEFT,
                                   20.,None,MathsShape::Polygon(5,0.),
                                   &pal.red));
        closure_add(lc,&fix_mathsshape(&cedge(TOPRIGHT,cpixel(30,30)),
                                   A_TOPRIGHT,
                                   20.,None,MathsShape::Polygon(5,0.),
                                   &pal.red));
        closure_add(lc,&fix_mathsshape(&cedge(BOTTOMLEFT,cpixel(30,30)),
                                   A_BOTTOMLEFT,
                                   20.,None,MathsShape::Polygon(5,0.),
                                   &pal.red));
        closure_add(lc,&fix_mathsshape(&cedge(BOTTOMRIGHT,cpixel(30,30)),
                                   A_BOTTOMRIGHT,
                                   20.,None,MathsShape::Polygon(5,0.),
                                   &pal.red));
        closure_done(lc);
    }})
}

#[derive(Clone)]
struct Palette {
    col: Colour,
    green_col: Colour,
    green: ColourSpec,
    red: ColourSpec,
    middle: i32,
    fc_font: FCFont
}

impl Palette {
    fn new() -> Palette {
        let fc_font = FCFont::new(12,"Lato",FontVariety::Normal);

        let mut middle = SH / 120;
        if middle < 5 { middle = 5; }
        
        let green_col = Colour(50,255,150);
        let green = ColourSpec::Spot(green_col);
        let red = ColourSpec::Spot(Colour(255,100,50));
        Palette {
            col: Colour(200,200,200),
            green_col, green, red, middle, fc_font
        }
    }
}
