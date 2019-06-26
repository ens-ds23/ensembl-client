// We keep these separate from the other utils partly because these imports
// are very hairy.

use itertools::Itertools;
use serde_json::Value as JSONValue;
use stdweb::Value;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document, Element, HtmlElement, IHtmlElement,
    IElement, IParentNode, INode
};
use stdweb::web::html_element::CanvasElement;

use dom::webgl::{
    WebGLRenderingContext as glctx,
};

use types::{ area, Dot, Rect };

fn _to_html(el: Option<Element>) -> Option<HtmlElement> {
    el.and_then(|el| el.try_into().ok())
}

pub fn query_selector_ok(root: &HtmlElement, sel: &str, message: &str) -> HtmlElement {
    let x : Option<HtmlElement> = _to_html(ok!(root.query_selector(sel)));
    ok!(x.ok_or_else(|| message.to_string()))
}

pub fn query_selector_ok_doc(sel: &str, message: &str) -> HtmlElement {
    let root : HtmlElement = document().document_element()
            .expect("No root element").try_into().expect("no root element");
    query_selector_ok(&root,sel,message)
}

pub fn query_selector_new(sel: &str) -> Option<HtmlElement> {
    if let Some(Some(el)) = document().query_selector(sel).ok() {
        el.try_into().ok()
    } else {
        None
    }
}

pub fn query_selector2(root: &HtmlElement, sel: &str) -> Option<HtmlElement> {
    if let Ok(Some(el)) = root.query_selector(sel) {
        el.try_into().ok()
    } else {
        None
    }
}

pub fn in_page(el: &HtmlElement) -> bool {
    let b = js! {
        return document.body.contains(@{el.as_ref()});
    };
    b.try_into().unwrap()
}

#[deprecated(note="use query_selector_ok instead")]
pub fn query_selector(el: &Element, sel: &str) -> Element {
    el.query_selector(sel).unwrap().unwrap()
}

pub fn query_select(sel: &str) -> Element {
    document().query_selector(sel).unwrap().unwrap()
}

pub fn size(el: &HtmlElement) -> Dot<f64,f64> {
    let r = el.get_bounding_client_rect();
    Dot(r.get_width(),r.get_height())
}

pub fn window_size() -> Dot<f64,f64> {
    let v : Vec<f64> = js! { 
        return [document.documentElement.clientWidth,
                document.documentElement.clientHeight];
    }.try_into().unwrap();
    Dot(v[0],v[1])
}

pub fn position(el: &HtmlElement) -> Rect<f64,f64> {
    let r = el.get_bounding_client_rect();
    area(Dot(r.get_left(),r.get_top()),
         Dot(r.get_right(),r.get_bottom()))
}

pub fn window_space(el: &HtmlElement) -> Rect<f64,f64> {
    let pos = position(el).rectangle();
    let wsz = window_size();
    area(pos[0],Dot(wsz.0-pos[2].0,wsz.1-pos[2].1))
}

pub fn add_attr(el: &HtmlElement,key: &str, more: &str) {
    let val = match el.get_attribute(key) {
        Some(x) => x+" ",
        None => "".to_string(),
    } + more;
    el.set_attribute(key,&val).ok();
}

pub fn remove_attr(el: &HtmlElement,key: &str, togo: &str) {
    let val = match el.get_attribute(key) {
        Some(x) => x,
        None => "".to_string(),
    };
    let val = val.split(" ").filter(|x| *x != togo).join(" ");
    el.set_attribute(key,&val).ok();
}

#[allow(unused)]
pub fn add_style(el: &HtmlElement, key: &str, value: &str) {
    add_attr(el,"style",&format!("{}: {};",key,value));
}

pub fn get_inner_html(el: &Element) -> String {
    return js! { return @{el.as_ref()}.innerHTML; }.try_into().unwrap();
}

pub fn inner_html(el: &HtmlElement, value: &str) {
    js! { @{el.as_ref()}.innerHTML = @{value} };
}

pub fn text_content(el: &HtmlElement, value: &str) {
    js! { @{el.as_ref()}.textContent = @{value} };
}

pub fn append_element(el: &HtmlElement, name: &str) -> HtmlElement {
    let doc = el.owner_document().unwrap();
    let new = doc.create_element(name).ok().unwrap();
    el.append_child(&new);
    unwrap!(new.try_into())
}

pub fn remove(el: &HtmlElement) {
    let parent: Element = el.parent_node().unwrap().try_into().ok().unwrap();
    parent.remove_child(el).unwrap_or(el.clone().into());
}

pub fn scroll_to_bottom(el: &Element) {
    js! { @{el.as_ref()}.scrollTop = @{el.as_ref()}.scrollHeight; };
}

pub fn send_custom_event(el: &HtmlElement, name: &str, data: &JSONValue) {
    let v: Value = data.clone().try_into().unwrap();
    js! {
        var e = new CustomEvent(@{name},{ detail: @{&v}, bubbles: true });
        @{el.as_ref()}.dispatchEvent(e);
    };
}

pub fn clear_selection() {
    js! {
        var sel = window.getSelection ? window.getSelection() : document.selection;
        if (sel) {
            if (sel.removeAllRanges) {
                sel.removeAllRanges();
            } else if (sel.empty) {
                sel.empty();
            }
        }
    }
}

pub fn get_context(canvas: &CanvasElement) -> glctx {
    (js! {
        return @{canvas.as_ref()}.getContext("webgl",{ antialias: true });
    }).try_into().ok().unwrap()
}

pub fn browser_time() -> f64 {
    return  js! { return +new Date(); }.try_into().unwrap();
}

/* Not sure why this isn't implemented in stdweb */
#[allow(unused)]
pub fn get_classes(el: &HtmlElement) -> Vec<String> {
    let raw = el.class_list();
    let parts : Vec<String> = js! { return Array.from(@{raw}.values()); }.try_into().ok().unwrap();
    parts
}
