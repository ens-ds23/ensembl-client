use model::shape::{ TextArtist, DrawingHash };
use super::{ Artist, FlatCanvas };
use types::{ Colour, CPixel };

impl Artist for TextArtist {
    fn draw(&self, canvs: &FlatCanvas, pos: CPixel) {
        canvs.text(&self.chars,pos,&self.font, &self.colour, &self.background);
    }
        
    fn draw_mask(&self, canvs: &FlatCanvas, pos: CPixel) {
        canvs.text(&self.chars,pos,&self.font, &Colour(0,0,0), &Colour(255,255,255));
    }
    
    fn memoize_key(&self) -> Option<DrawingHash> {
        Some(DrawingHash::new(( &self.chars, &self.font, &self.colour )))
    }
    
    fn measure(&self, canvas: &FlatCanvas) -> CPixel {
        canvas.measure(&self.chars,&self.font)
    }
    
    fn measure_mask(&self, canvas: &FlatCanvas) -> CPixel {
        self.measure(canvas)
    }
}
