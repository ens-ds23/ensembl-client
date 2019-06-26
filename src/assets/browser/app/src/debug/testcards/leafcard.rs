use util::RulerGenerator;
use debug::support::closuresource::{ ClosureSource, closure_add, closure_done, closure_add_opt };
use composit::Source;
use drivers::webgl::{
    FCFont, FontVariety,
    Facade, ShapeShortInstanceData
};
use model::shape::{
    PinRectTypeSpec, StretchRectTypeSpec, text_texture,
    TextureTypeSpec, 
};
use types::{ Colour, cleaf, cpixel, A_TOP, area, area_size, AxisSense };

const TARGET: i32 = 10;
const TICK_TARGET: i32 = 50;
const MARK_TARGET: i32 = 200;

pub fn leafcard_source(leaf_marks: bool) -> impl Source {
    let font = FCFont::new(16,"Lato",FontVariety::Normal);
    ClosureSource::new(0.2,move |lc,leaf| {
        let rg = RulerGenerator::new_leaf(leaf);
        let ruler = rg.ruler(MARK_TARGET,TICK_TARGET,TARGET,&[10,15,20,30]);
        
        let prts = PinRectTypeSpec {
            sea_x: None,
            sea_y: None,
            ship_x: (Some(AxisSense::Min),0),
            ship_y: (Some(AxisSense::Min),0),
            under: 0,
            spot: false
        };
                
        for (offset,height,text) in ruler {
            if let Some(text) = text {
                let tx = text_texture(&text,&font,&Colour(199,208,213),&Colour(255,255,255));
                
                let tts = TextureTypeSpec {
                    sea_x: None,
                    sea_y: None,
                    ship_x: (None,0),
                    ship_y: (Some(AxisSense::Max),0),
                    under: 0,
                    scale_x: 1., scale_y: 1.
                };
                closure_add_opt(lc,&tts.new_short_shape(&ShapeShortInstanceData {
                    pos_x: offset,
                    pos_y: 41,
                    aux_x: 0.,
                    aux_y: 0,
                    facade: Facade::Drawing(tx)
                }));
            }
            closure_add_opt(lc,&prts.new_short_shape(&ShapeShortInstanceData {
                pos_x: offset,
                pos_y: 10,
                aux_x: 1.,
                aux_y: height,
                facade: Facade::Colour(Colour(199,208,213))
            }));
        }
        let i = leaf.get_index();
        let (colour,_) = if i % 2 == 0 {
            (Colour(255,0,0),0)
        } else {
            (Colour(0,255,0),10)
        };
        if leaf_marks {
            let srts = StretchRectTypeSpec { spot: false, hollow: false };
            closure_add_opt(lc,&srts.new_short_shape(&ShapeShortInstanceData {
                pos_x: 0.,
                pos_y: 70,
                aux_x: 1.,
                aux_y: 10,
                facade: Facade::Colour(colour)
            }));
            closure_done(lc);
        } else {
            closure_done(lc);
        }
    })  
}
