use types::{ CPixel, area_size, cpixel };
use super::{ FlatCanvas, CarriageCanvases, OneCanvasManager };
use model::shape::DrawingHash;
use super::super::program::CanvasWeave;

/* A Artist can service some class of DrawingImpls.
 * A texture type will create an instance of them. Note that a
 * Artist is not parameterised, it can draw exactly one thing.
 */
pub trait Artist {
    fn draw(&self, canv: &FlatCanvas, pos: CPixel);
    fn draw_mask(&self, canv: &FlatCanvas, pos: CPixel) {
        canv.bitmap(&vec!{ 
            0,0,0,255,0,0,0,255,0,0,0,255,
            0,0,0,255,0,0,0,255,0,0,0,255,
            0,0,0,255,0,0,0,255,0,0,0,255,
        }, area_size(pos,cpixel(3,3)));
    }
    fn memoize_key(&self) -> Option<DrawingHash>  { None }
    fn measure(&self, canv: &FlatCanvas) -> CPixel;
    fn measure_mask(&self, _canv: &FlatCanvas) -> CPixel { cpixel(1,1) }
    
    fn select_canvas<'a>(&self, ds: &'a mut CarriageCanvases) -> &'a mut OneCanvasManager {
        ds.get_ocm(CanvasWeave::Pixelate)
    }
    fn margin(&self) -> CPixel { cpixel(0,0) }
    fn padding(&self) -> CPixel { cpixel(0,0) }
}

pub trait Mark : Artist {
    fn get_offset(&self) -> CPixel;
}
