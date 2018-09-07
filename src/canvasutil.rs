use stdweb::web::{
    IHtmlElement,
    INode,
    TextBaseline,
    CanvasRenderingContext2d,
    document,
    Element
};

use stdweb::web::html_element::{
    CanvasElement
};

use stdweb::web::TypedArray;
use stdweb::unstable::TryInto;
use coord::{
    Colour, CPixel, RPixel
};

use dom::domutil;

// Prepare a canvas ready for WebGL
pub fn prepare_canvas(sel: &str) -> CanvasElement {
    // get canvas
    let canvasel: Element = domutil::query_select(sel);
    let canvas: CanvasElement = canvasel.try_into().unwrap();

    // force CSS onto attributes of canvas tag
    let width = canvas.offset_width() as u32;
    let height = canvas.offset_height() as u32;
    console!("offset {:?} x {:?}",width,height);
    let width = width - width % 2;
    let height = height - height % 2;
    canvas.set_width(width);
    canvas.set_height(height);
    // update CSS in px, as %'s are dodgy on canvas tags
    let mc : Element = domutil::query_select(sel);
    domutil::add_style(&mc,"width",&format!("{}px",width));
    domutil::add_style(&mc,"height",&format!("{}px",height));
    //window().add_event_listener(enclose!( (canvas) move |_:ResizeEvent| {
    //    canvas.set_width(canvas.offset_width() as u32);
    //    canvas.set_height(canvas.offset_height() as u32);
    //}));
    canvas
}

pub fn aspect_ratio(canvas: &CanvasElement) -> f32 {
    canvas.offset_width() as f32 / canvas.offset_height() as f32
}

#[derive(Clone,Eq,PartialEq,Hash)]
pub struct FCFont {
    spec: String,
    height: i32,
    xpad: i32,
    ypadtop: i32,
    ypadbot: i32
}

impl FCFont {
    pub fn new(size : i32,family: &str) -> FCFont {
        FCFont { spec: format!("{}px {}",size,family),
                 height: size, ypadtop: 0, ypadbot: 4, xpad: 0 }
    }
    
    fn setup(&self, canvas : &CanvasRenderingContext2d) {
        canvas.set_font(&self.spec);
    }
}

pub struct FlatCanvas {
    canvas: CanvasElement,
    context : CanvasRenderingContext2d,
    width: i32,
    height: i32,
}

impl FlatCanvas {        
    pub fn reset() {
        let ch = domutil::query_select("#managedcanvasholder");
        domutil::inner_html(&ch,"");        
    }
        
    pub fn create(width: i32,height: i32) -> FlatCanvas {
        let canvas_holder = domutil::query_select("#managedcanvasholder");
        let canvas : CanvasElement = document().create_element("canvas").ok().unwrap().try_into().unwrap();
        canvas_holder.append_child(&canvas);
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        let context : CanvasRenderingContext2d = canvas.get_context().unwrap();
        context.set_fill_style_color("white");
        context.fill_rect(0.,0.,width as f64,height as f64);
        context.set_fill_style_color("black");
        FlatCanvas { canvas, context, height, width }
    }
    
    pub fn text(&self,text : &str, pos: CPixel, font: &FCFont, col: &Colour) -> (i32,i32) {
        font.setup(&self.context);
        self.context.set_text_baseline(TextBaseline::Top);
        self.context.set_fill_style_color(&col.to_css()[..]);
        self.context.set_stroke_style_color(&col.to_css()[..]);
        self.context.fill_text(text,(pos.0+font.xpad).into(),(pos.1+font.ypadtop).into(),None);
        let m = self.context.measure_text(text);
        let width_px = m.unwrap().get_width().ceil() as i32;
        let height_px = font.height;
        (width_px+2*font.xpad,height_px+font.ypadtop+font.ypadbot)
    }
    
    pub fn bitmap(&self, data: &Vec<u8>, coords: RPixel) {
        let pixels: TypedArray<u8> = data[..].into();
        let RPixel(CPixel(x,y),CPixel(width,height)) = coords;
        js! {
            var id = @{&self.context}.createImageData(@{width},@{height});
            id.data.set(@{pixels});
            @{&self.context}.putImageData(id,@{x},@{y});
        };
    }
    
    pub fn rectangle(&self, coords: RPixel, col: &Colour) {
        let RPixel(CPixel(x,y),CPixel(w,h)) = coords;
        self.context.set_fill_style_color(&col.to_css()[..]);
        self.context.fill_rect(x as f64,y as f64,w as f64,h as f64);
    }

    pub fn measure(&self,text : &str, font: &FCFont) -> CPixel {
        font.setup(&self.context);
        let m = self.context.measure_text(text);
        let width_px = m.unwrap().get_width().ceil() as i32;
        let height_px = font.height;
        CPixel(width_px+2*font.xpad,height_px+font.ypadtop+font.ypadbot)
    }
    
    pub fn element(&self) -> &CanvasElement {
        &self.canvas
    }
    
    pub fn size(&self) -> CPixel {
        CPixel(self.width,self.height)
    }
    
    pub fn prop_x(&self,x: i32) -> f32 {
        (x as f64 / self.width as f64) as f32
    }

    pub fn prop_y(&self,y: i32) -> f32 {
        (y as f64 / self.height as f64) as f32
    }

}
