use stdweb::unstable::TryInto;

use stdweb::web::HtmlElement;

use dom::{ domutil, Bling };

pub const PLAINSTAGE : &str = r##"
<div class="bpane-container">
    <div class="bpane-canv">
    </div>
    <div class="managedcanvasholder"></div>
</div>
"##;

pub const PLAINSTAGE_CSS : &str = r##"
.bpane-container {
  position: relative;
}

.bpane-canv {
  position: absolute;
  top: 0;
  left: 0;
  overflow: hidden;
}

.bpane-container .managedcanvasholder {
    display: none;
}

.bpane-canv canvas {
  margin: 0;
  display:block;
}

.bpane-container, .bpane-container .bpane-canv {
    height: 100%;
    width: 100%;
}

@import url('https://fonts.googleapis.com/css?family=Lato');
"##;

pub struct NoBling {}

impl NoBling {
    pub fn new() -> NoBling { NoBling{} }
}

impl Bling for NoBling {
    fn apply_bling(&self, el: &HtmlElement) -> HtmlElement {
        let el = el.clone().into();
        domutil::inner_html(&el,PLAINSTAGE);
        if let Some(old) = domutil::query_selector2(&el,"#bpane-css") {
            domutil::remove(&old);
        }       
        let css = domutil::append_element(&domutil::query_selector_ok_doc("head","no head element"),"style");
        domutil::add_attr(&css,"id","bpane-css");
        domutil::inner_html(&css,PLAINSTAGE_CSS);
        domutil::query_selector(&el.clone().into(),".bpane-canv").clone().try_into().unwrap()
    }
}
