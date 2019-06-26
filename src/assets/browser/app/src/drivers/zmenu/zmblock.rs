use std::collections::{ HashMap, HashSet };
use std::iter::Peekable;
use std::str::Chars;

use serde_json::Map as JSONMap;
use serde_json::Value as JSONValue;

#[derive(Clone,Debug)]
enum ZMenuText {
    Fixed(String),
    Template(String)
}

#[derive(Clone)]
enum ZMenuTag {
    Text(ZMenuText),
    MarkupOn(String),
    MarkupOff(String)
}

#[derive(Clone,Debug)]
struct ZMenuItem {
    text: ZMenuText,
    markup: Vec<String>
}

impl ZMenuItem {
    fn new(text: ZMenuText, markup: Vec<String>) -> ZMenuItem {
        ZMenuItem { text, markup }
    }
    
    fn apply(&self, data: &HashMap<String,String>) -> JSONValue {
        let text = match &self.text {
            ZMenuText::Fixed(s) => s.to_string(),
            ZMenuText::Template(k) => data.get(k).cloned().unwrap_or("".to_string())
        };
        json!({
            "text": text,
            "markup": JSONValue::Array(self.markup.iter().map(|x|
                            JSONValue::String(x.to_string())
                        ).collect())
        })
    }
}

fn push_fixed(out: &mut Vec<ZMenuTag>, fixed: &mut String) {
    if fixed.len() > 0 {
        out.push(ZMenuTag::Text(ZMenuText::Fixed(fixed.to_string())));
        fixed.clear();
    }
}

fn get_tag(chars: &mut Peekable<Chars>, term: char) -> String {
    let mut out = String::new();
    let mut bs = false;
    loop {
        if let Some(c) = chars.next() {
            if bs {
                out.push(c);
                bs = false;
            } else if c == term {
                break;
            } else if c == '\\' {
                bs = true;
            } else {
                out.push(c);
            }
        } else {
            break;
        }
    }
    out
}

fn fmt_parse_block(chars: &mut Peekable<Chars>) -> Vec<ZMenuTag> {
    let mut out = Vec::new();
    let mut bs = false;
    
    let mut fixed = String::new();
    while let Some(c) = chars.peek() {
        if bs {
            fixed.push(chars.next().unwrap());
            bs = false;
        } else if c == &'{' {
            push_fixed(&mut out,&mut fixed);
            chars.next();
            out.push(ZMenuTag::Text(ZMenuText::Template(get_tag(chars,'}'))));
        } else if c == &'<' {
            push_fixed(&mut out,&mut fixed);
            chars.next();
            if chars.peek() == Some(&'/') {
                chars.next();
                out.push(ZMenuTag::MarkupOff(get_tag(chars,'>')));
            } else {
                out.push(ZMenuTag::MarkupOn(get_tag(chars,'>')));
            }
        } else if c == &']' {
            break;
        } else if c == &'\\' {
            bs = true;
        } else {
            fixed.push(chars.next().unwrap());
        }
    }
    push_fixed(&mut out,&mut fixed);
    out
}

#[derive(Debug,Clone)]
struct ZMenuBlock {
    items: Vec<ZMenuItem>
}

impl ZMenuBlock {
    fn make_items(tags: Vec<ZMenuTag>) -> Vec<ZMenuItem> {
        let mut markup = HashSet::new();
        let mut items = Vec::new();
        for tag in tags {
             match tag {
                ZMenuTag::Text(text) => {
                    let markup_vec = markup.iter().cloned().collect();
                    items.push(ZMenuItem::new(text.clone(),markup_vec));
                },
                ZMenuTag::MarkupOn(m) => {
                    markup.insert(m.to_string());
                },
                ZMenuTag::MarkupOff(m) => {
                    markup.remove(&m);
                }
            }
        }
        items
    }
        
    fn new(chars: &mut Peekable<Chars>) -> ZMenuBlock {
        ZMenuBlock {
            items: ZMenuBlock::make_items(fmt_parse_block(chars))
        }
    }
    
    fn apply(&self, data: &HashMap<String,String>) -> JSONValue {
        JSONValue::Array(self.items.iter().map(|x| x.apply(data)).collect())
    }
}

#[derive(Debug,Clone)]
enum ZMenuSequence {
    Item(ZMenuBlock),
    LineBreak
}

#[derive(Debug,Clone)]
pub struct ZMenuFeatureTmpl {
    items: Vec<ZMenuSequence>
}

fn fmt_parse_feature(spec: &str) -> Vec<ZMenuSequence> {
    let mut chars = spec.chars().peekable();
    let mut out = Vec::new();
    while let Some(c) = chars.next() {
        if c == '[' {
            out.push(ZMenuSequence::Item(ZMenuBlock::new(&mut chars)));
            chars.next(); // ]
        } else if c == '/' {
            out.push(ZMenuSequence::LineBreak);
        }
    }
    out
}

impl ZMenuFeatureTmpl {
    pub fn new(spec: &str) -> ZMenuFeatureTmpl {
        ZMenuFeatureTmpl {
            items: fmt_parse_feature(spec)
        }
    }
    
    fn build(&self, data: &HashMap<String,String>) -> Vec<Vec<JSONValue>> {
        let mut out = Vec::new();
        out.push(Vec::new());
        for item in &self.items {
            match item {
                ZMenuSequence::Item(block) => out.last_mut().unwrap().push(block.apply(data)),
                ZMenuSequence::LineBreak => out.push(Vec::new())
            }
        }
        out
    }
    
    fn convert(&self, input: Vec<Vec<JSONValue>>) -> JSONValue {
        JSONValue::Array(input.iter().map(|x| JSONValue::Array(x.to_vec())).collect())
    }
    
    pub fn apply(&self, id: &str, data: &HashMap<String,String>) -> JSONValue {
        json!({
            "id": id.to_string(),
            "lines": self.convert(self.build(data))
        })
    }
}
