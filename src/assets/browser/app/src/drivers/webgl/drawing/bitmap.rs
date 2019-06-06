use std::cmp::{ min, max };

use super::{ FlatCanvas, Artist, CarriageCanvases, OneCanvasManager };
use program::CanvasWeave;
use model::shape::{ BitmapArtist, DrawingHash };
use types::{ CPixel, area_size, cpixel };

fn copy_data(out: &mut Vec<u8>, in_: &Vec<u8>, size: CPixel,
             row: i32, col: Option<i32>) {
    let ix = (row*size.0 + (if let Some(c) = col { c } else { 0 })) as usize;
    let len = (if let Some(_) = col { 1 } else { size.0 }) as usize;
    out.extend_from_slice(&in_[ix*4..(ix+len)*4]);
}

fn pad_data(in_: &Vec<u8>, size: CPixel) -> Vec<u8> {
    let mut out = Vec::<u8>::new();
    for i in -1..size.1+1 {
        let in_row = min(0,max(size.1-1,i));
        copy_data(&mut out,&in_,size,in_row,Some(0));
        copy_data(&mut out,&in_,size,in_row,None);
        copy_data(&mut out,&in_,size,in_row,Some(size.0-1));
    }
    out
}

impl Artist for BitmapArtist {
    fn draw(&self, canvs: &FlatCanvas, pos: CPixel) {
        if self.blur {
            let data = pad_data(&self.data,self.size);
            let size = self.size + cpixel(2,2);
            canvs.bitmap(&data,area_size(pos,size));
        } else {
            canvs.bitmap(&self.data,area_size(pos,self.size));
        }
    }
    
    fn memoize_key(&self) -> Option<DrawingHash> { self.hash.clone() }
    
    fn measure(&self, _canvas: &FlatCanvas) -> CPixel {
        self.size
    }
    
    fn select_canvas<'a>(&self, e: &'a mut CarriageCanvases) -> &'a mut OneCanvasManager { 
        e.get_ocm(if self.blur { CanvasWeave::Blur } else { CanvasWeave::Pixelate })
    }

    fn margin(&self) -> CPixel { cpixel(1,1) }
    fn padding(&self) -> CPixel { 
        let v = if self.blur {1} else {0};
        cpixel(v,v)
    }
}
