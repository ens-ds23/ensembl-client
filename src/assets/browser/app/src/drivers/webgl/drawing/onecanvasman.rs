use std::rc::Rc;
use std::collections::HashMap;

use types::{ CPixel, RPixel, area_size, cpixel };
use super::alloc::{ Ticket, Allocator };
use super::{ FlatCanvas, Drawing, Artist,  AllCanvasAllocator };
use super::super::program::CanvasWeave;
use model::shape::DrawingHash;

struct DrawingMemory {
    cache: HashMap<u64,Drawing>
}

impl DrawingMemory {
    fn new() -> DrawingMemory {
        DrawingMemory {
            cache: HashMap::<u64,Drawing>::new()
        }
    }

    fn insert(&mut self, key: &DrawingHash, val: Drawing) {
        self.cache.insert(key.get(),val);
    }
    
    fn lookup(&self, a: &Rc<Artist>) -> (Option<DrawingHash>,Option<Drawing>) {
        let tdrk = a.memoize_key();
        if let Some(tdrk) = tdrk {
            if let Some(obj) = self.cache.get(&tdrk.get()) {
                // can cache, is cached
                (Some(tdrk),Some(obj.clone()))
            } else {
                // can cache, isn't cached
                (Some(tdrk),None)
            }
        } else {
            // Can't cache
            (None,None)
        }
    }
}

/* Manages DrawingImpls. Single instance in Leaf. Just stores
 * them and calls draw when ready really. Little more than a fancy Vec.
 */
pub struct OneCanvasManager {
    pub canvas: Option<FlatCanvas>,
    standin: FlatCanvas,
    cache: DrawingMemory,
    drawings: Vec<Drawing>,
    allocator: Allocator,
    index: u32,
    ds_idx: u32,
    weave: CanvasWeave
}

impl OneCanvasManager {
    pub fn new(ds_idx: u32, canv_idx: u32, weave: CanvasWeave, standin: &FlatCanvas) -> OneCanvasManager {
        OneCanvasManager {
            canvas: None,
            ds_idx,
            index: canv_idx,
            standin: standin.clone(),
            cache: DrawingMemory::new(),
            drawings: Vec::<Drawing>::new(),
            allocator: Allocator::new(20),
            weave
        }
    }

    pub fn add_request(&mut self, a: Rc<Artist>) -> Drawing {
        let (tdrk,val) = self.cache.lookup(&a);
        if let Some(tdrh) = val {
            // already in cache
            tdrh
        } else {
            let margin = a.margin();
            let padding = a.padding();
            let size = a.measure(&self.standin);
            let mask_size = a.measure_mask(&self.standin);
            let flat_alloc = &mut self.allocator;
            let req = flat_alloc.request(size + cpixel(2,2)*(margin+padding));
            let mask_req = flat_alloc.request(mask_size + cpixel(2,2));
            let val = Drawing::new(a,req,mask_req);
            if let Some(tdrk) = tdrk {
                // put in cache
                self.cache.insert(&tdrk,val.clone());
            }
            self.drawings.push(val.clone());
            val
        }
    }

    pub fn draw(&mut self, aca: &mut AllCanvasAllocator, canvs: FlatCanvas) {
        if let Some(ref old) = self.canvas {
            old.remove(aca);
        }
        self.canvas = Some(canvs);
        for tr in &self.drawings {
            tr.draw(self);
        }
    }
    
    pub fn get_full_idx(&self) -> (u32,u32) { (self.ds_idx,self.index) }
    
    pub fn allocate(&mut self) -> CPixel {
        self.allocator.allocate()
    }
    
    pub fn ticket_pos(&self, t: &Ticket) -> CPixel {
        self.allocator.position(&t)
    }
    
    pub fn ticket_size(&self, t: &Ticket) -> RPixel {
        let size = self.allocator.size(t);
        let pos = self.allocator.position(t);
        area_size(pos,size)
    } 
    
    pub fn destroy(&self, aca: &mut AllCanvasAllocator) {
        if let Some(ref fc) = self.canvas {
            fc.remove(aca);
        }
    }

    pub fn index(&self) -> u32 { self.index }
    pub fn weave(&self) -> CanvasWeave { self.weave }
}
