use std::rc::Rc;

use types::{ CPixel, RPixel, RFraction, cpixel, area };
use super::alloc::Ticket;
use super::{ Artist, OneCanvasManager };
use program::CanvasWeave;
use drivers::webgl::GLProgData;

pub struct Artwork {
    pub pos: RFraction,
    pub mask_pos: RFraction,
    pub size: CPixel,
    pub weave: CanvasWeave
}

/* One request to draw on the backing canvas. A combination of
 * Artist and a ticket (to get a location, when ready)
 */
pub struct DrawingImpl {
    gen: Rc<Artist>,
    ticket: Ticket,
    mask_ticket: Ticket,
    margin: CPixel,
    padding: CPixel
}

#[derive(Clone)]
pub struct Drawing(Rc<DrawingImpl>);

impl Drawing {
    pub fn new(gen: Rc<Artist>, ticket: Ticket, mask_ticket: Ticket) -> Drawing {
        let margin = gen.margin();
        let padding = gen.padding();
        Drawing(
            Rc::new(DrawingImpl {
                gen, ticket, mask_ticket, margin, padding
            }))
    }

    pub fn draw(&self, src: &OneCanvasManager) {
        let pos = src.ticket_pos(&self.0.ticket);
        self.0.gen.draw(&mut src.canvas.as_ref().unwrap(),pos + self.0.margin);
        let mask_pos = src.ticket_pos(&self.0.mask_ticket);
        self.0.gen.draw_mask(&mut src.canvas.as_ref().unwrap(),mask_pos + cpixel(1,1));
    }

    pub fn artwork(&self, e: &mut GLProgData) -> Artwork {
        let src = self.0.gen.select_canvas(e.get_canvases_mut());
        let canvas = src.canvas.as_ref().unwrap();
        let cs = canvas.size().as_fraction();
        let inset = self.0.margin + self.0.padding;
        let m = self.measure(src).inset(area(inset,inset));
        let mm = self.measure_mask(src).inset(area(cpixel(1,1),cpixel(1,1)));
        Artwork {
            pos: m.as_fraction() / cs,
            mask_pos: mm.as_fraction() / cs,
            size: m.area(),
            weave: src.weave()
        }
    }
        
    pub fn measure(&self, src: &OneCanvasManager) -> RPixel {
        src.ticket_size(&self.0.ticket)
    }

    pub fn measure_mask(&self, src: &OneCanvasManager) -> RPixel {
        src.ticket_size(&self.0.mask_ticket)
    }
}
