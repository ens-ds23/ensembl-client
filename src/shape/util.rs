use std::iter;
use program::{
    ProgramAttribs, DataBatch, DataGroup, ProgramType, PTMethod, 
    PTGeom, PTSkin
};
use types::{ RFraction, CLeaf, RPixel, RLeaf, cleaf, RCorner };
use shape::ColourSpec;
use program::Input;

pub fn triangle_gl(b: DataBatch, pdata: &mut ProgramAttribs, key: &str, p: &[&Input;3]) {
    if let Some(obj) = pdata.get_object(key) {
        obj.add_data(&b,&[p[0], p[1], p[2]]);
    }
}

pub fn rectangle_g(b: DataBatch, pdata: &mut ProgramAttribs, key: &str, p: &RLeaf) {
    if let Some(obj) = pdata.get_object(key) {
        obj.add_data(&b,&[p]);
    }
}

pub fn rectangle_p(b: DataBatch, pdata: &mut ProgramAttribs, key: &str, p: &RPixel) {
    if let Some(obj) = pdata.get_object(key) {
        obj.add_data(&b,&[p]);
    }
}

pub fn rectangle_c(b: DataBatch, pdata: &mut ProgramAttribs, 
                   key: &str, key_sgn: &str, p: &RCorner) {
    rectangle_p(b,pdata,key,&p.quantity());
    if let Some(obj) = pdata.get_object(key_sgn) {
        obj.add_data(&b,&[&p.corners()]);
    }
}

pub fn poly_p(b: DataBatch, pdata: &mut ProgramAttribs, key: &str, p: &[&Input]) {
    if let Some(obj) = pdata.get_object(key) {
        obj.add_data(&b,p);
    }
}

pub fn rectangle_t(b: DataBatch, pdata: &mut ProgramAttribs, key: &str, p: &RFraction) {
    if let Some(obj) = pdata.get_object(key) {
        obj.add_data(&b,&[p]);
    }
}

pub fn multi_gl(b: DataBatch, pdata: &mut ProgramAttribs, key: &str, d: &Input, mul: u16) {
    let mut v = Vec::<&Input>::new();
    v.extend(iter::repeat(d).take(mul as usize));
    if let Some(obj) = pdata.get_object(key) {
        obj.add_data(&b,&v.as_slice());
    }
}

fn group(pdata: &ProgramAttribs, g: Option<DataGroup>) -> DataGroup {
    g.unwrap_or_else(|| pdata.get_default_group())
}

pub fn vertices_rect(pdata: &mut ProgramAttribs, g: Option<DataGroup>) -> DataBatch {
    let g = group(pdata,g);
    pdata.add_vertices(g,&[0,3,1,2,1,3],4)
}

pub fn vertices_tri(pdata: &mut ProgramAttribs, g: Option<DataGroup>) -> DataBatch {
    let g = group(pdata,g);
    pdata.add_vertices(g,&[0,1,2],3)
}

pub fn vertices_poly(pdata: &mut ProgramAttribs, n: u16, g: Option<DataGroup>) -> DataBatch {
    let g = group(pdata,g);
    let mut v = Vec::<u16>::new();
    
    for i in 0..n-1 {
        v.push(0);
        v.push(i+1);
        v.push(i+2);
    }
    v.push(0);
    v.push(n);
    v.push(1);
    pdata.add_vertices(g,&v,n+1)
}


pub fn vertices_hollowpoly(pdata: &mut ProgramAttribs, n: u16, g: Option<DataGroup>) -> DataBatch {
    let g = group(pdata,g);
    let mut v = Vec::<u16>::new();
    v.push(0);
    for i in 0..n*2 {
        v.push(i);
    }
    v.push(0);
    v.push(1);
    v.push(1);
    pdata.add_vertices(g,&v,n*2)
}

pub fn vertices_strip(pdata: &mut ProgramAttribs, len: u16, g: Option<DataGroup>) -> DataBatch {
    let g = group(pdata,g);
    let mut v = Vec::<u16>::new();
    for i in 0..len {
        v.push(i);
    }
    pdata.add_vertices(g,&v,len)
}

pub fn points_g(b: DataBatch, pdata: &mut ProgramAttribs, key: &str, p_in: &[CLeaf], y: i32) {
    if let Some(obj) = pdata.get_object(key) {
        if let Some(v) = p_in.first() { // double first for strip break
            obj.add_data(&b,&[v]);
        }
        for p in p_in {
            let q = *p + cleaf(0.,y);
            obj.add_data(&b,&[p,&q]);
        }
        if let Some(v) = p_in.last() { // double last for strip break
            let q = *v + cleaf(0.,y);
            obj.add_data(&b,&[&q]);
        }
    }
}

pub fn despot(gt: PTGeom, mt: PTMethod, spec: &ColourSpec) -> ProgramType {
    let st = if let ColourSpec::Spot(_) = spec {
        PTSkin::Spot
    } else {
        PTSkin::Colour
    };
    ProgramType(gt,mt,st)
}
