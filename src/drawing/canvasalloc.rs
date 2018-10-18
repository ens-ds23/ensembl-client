use std::collections::HashMap;

use stdweb::web::{ Element, INode, document };
use stdweb::web::html_element::CanvasElement;
use stdweb::unstable::TryInto;

use composit::Leaf;
use dom::domutil;
use drawing::{  DrawingSession, FlatCanvas };
use program::CanvasWeave;
use types::{ Dot, cpixel };

#[derive(Debug)]
pub struct CanvasRemover(CanvasElement,u32);

impl CanvasRemover {
    pub fn remove(&self, aca: &mut AllCanvasAllocator) {
        aca.remove(&self.0,self.1);
    }
}

fn new_canvas(root: &Element) -> CanvasElement {
    let canvas : CanvasElement = 
        document().create_element("canvas")
            .ok().unwrap().try_into().unwrap();
    root.append_child(&canvas);
    canvas
}

fn free_canvas(el: &CanvasElement) {
    el.parent_node().unwrap().remove_child(el).ok();
}

pub struct AllCanvasAllocator {
    root: Element,
    idx: u32,
    canvases: HashMap<u32,FlatCanvas>,
    standin: FlatCanvas,
    standin_el: CanvasElement
}

impl AllCanvasAllocator {
    pub fn new(id: &str) -> AllCanvasAllocator {
        let root = domutil::query_select(id);
        let standin_el = new_canvas(&root);
        let standin = FlatCanvas::create(
                standin_el.clone(),2,2,&CanvasWeave::Pixelate,
                CanvasRemover(standin_el.clone(),0));
        AllCanvasAllocator {
            canvases: HashMap::<u32,FlatCanvas>::new(),
            standin, root, standin_el,
            idx: 0,
        }
    }
    
    pub fn finish(&mut self) {
        free_canvas(&self.standin_el);
    }
    
    fn remove(&mut self, el: &CanvasElement, idx: u32) {
        free_canvas(el);
        self.canvases.remove(&idx);
    }
    
    pub fn get_standin(&self) -> &FlatCanvas { &self.standin }
    
    pub fn flat_allocate(&mut self, size: Dot<i32,i32>, 
                         w: &CanvasWeave) -> FlatCanvas {
        let canvas_el = new_canvas(&self.root);
        let rm = CanvasRemover(canvas_el.clone(),self.idx);
        let canvas = FlatCanvas::create(canvas_el,size.0,size.1,w,rm);
        self.canvases.insert(self.idx,canvas.clone());
        self.idx += 1;
        canvas
    }
    
    pub fn make_drawing_session(&mut self) -> DrawingSession {
        DrawingSession::new(self)
    }    
}
