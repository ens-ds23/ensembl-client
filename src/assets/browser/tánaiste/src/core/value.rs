use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::char;

pub enum ValueImpl {
    Bytes(Vec<u8>),
    Float(Vec<f64>),
    String(Vec<String>)
}

impl ValueImpl {
    pub fn as_bytes<F,R>(&self, cb: F) -> R
                where F: FnOnce(&Vec<u8>) -> R {
        match self {
            ValueImpl::Bytes(b) =>  cb(&b),
            ValueImpl::Float(f) =>  cb(&float_to_bytes(&f)),
            ValueImpl::String(s) => cb(&string_to_bytes(&s))
        }
    }
    
    pub fn as_string<F,R>(&self, cb: F) -> R 
                where F: FnOnce(&Vec<String>) -> R {
        match self {
            ValueImpl::Bytes(b)  => cb(&bytes_to_string(&b)),
            ValueImpl::Float(f)  => cb(&float_to_string(&f)),
            ValueImpl::String(s) => cb(&s)
        }
    }
    
    pub fn as_floats<F,R>(&self, cb: F) -> R
                where F: FnOnce(&Vec<f64>) -> R {
        match self {
            ValueImpl::Bytes(b)  => cb(&bytes_to_float(&b)),
            ValueImpl::Float(f)  => cb(&f),
            ValueImpl::String(s) => cb(&string_to_float(&s))
        }        
    }
    
    pub fn len(&self) -> usize {
        match self {
            ValueImpl::Bytes(b) => b.len(),
            ValueImpl::Float(f) => f.len(),
            ValueImpl::String(s) => 
            {
                let sum: usize = s.iter().map(|x| x.len()).sum();
                s.len() + sum
            }
        }
    }
}

fn float_to_string(data: &Vec<f64>) -> Vec<String> {
    vec![data.iter().map(|s| char::from_u32(*s as u32).unwrap_or('\u{FFFD}')).collect()]
}

fn string_to_float(data: &Vec<String>) -> Vec<f64> {
    data.iter().map(|s|
        s.chars().nth(0).map(|x| x as u32 as f64).unwrap_or(0.)
    ).collect()
}

fn float_to_bytes(data: &Vec<f64>) -> Vec<u8> {
    string_to_bytes(&float_to_string(data))
}

fn bytes_to_float(data: &Vec<u8>) -> Vec<f64> {
    string_to_float(&bytes_to_string(data))
}

fn string_to_bytes(data: &Vec<String>) -> Vec<u8> {
    data[0].as_bytes().to_vec()
}

fn bytes_to_string(data: &Vec<u8>) -> Vec<String> {
    vec![String::from_utf8_lossy(data).to_string()]
}

#[derive(Clone)]
pub struct Value(Rc<RefCell<ValueImpl>>);

impl Value {
    pub fn new_null() -> Value {
        Value::new_from_float(vec!{})
    }

    pub fn new_from_bytes(data: Vec<u8>) -> Value {
        Value(Rc::new(RefCell::new(ValueImpl::Bytes(data))))
    }
    
    pub fn new_from_float(data: Vec<f64>) -> Value {
        Value(Rc::new(RefCell::new(ValueImpl::Float(data))))
    }
    
    pub fn new_from_string(data: Vec<String>) -> Value {
        Value(Rc::new(RefCell::new(ValueImpl::String(data))))
    }

    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    pub fn as_string<F,R>(&self, cb: F) -> R 
                where F: FnOnce(&Vec<String>) -> R {
        self.0.borrow().as_string(cb)                    
    }

    pub fn as_bytes<F,R>(&self, cb: F) -> R
                where F: FnOnce(&Vec<u8>) -> R {
        self.0.borrow().as_bytes(cb)
    }

    pub fn as_floats<F,R>(&self, cb: F) -> R
                where F: FnOnce(&Vec<f64>) -> R {
        self.0.borrow().as_floats(cb)
    }

    fn update(&mut self, v: Option<ValueImpl>) {
        if let Some(v) = v {
            *self.0.borrow_mut() = v;
        }
    }
        
    pub fn coerce_to_string(&mut self) {
        let v = {
            match *self.0.borrow() {
                ValueImpl::Float(ref f) =>
                    Some(ValueImpl::String(float_to_string(&f))),
                ValueImpl::String(_) => None,
                ValueImpl::Bytes(ref b) =>
                    Some(ValueImpl::String(bytes_to_string(&b)))
            }
        };
        self.update(v);
    }

    pub fn coerce_to_float(&mut self) {
        let v = {
            match *self.0.borrow() {
                ValueImpl::String(ref s) =>
                    Some(ValueImpl::Float(string_to_float(&s))),
                ValueImpl::Float(_) => None,
                ValueImpl::Bytes(ref b) =>
                    Some(ValueImpl::Float(bytes_to_float(&b)))
            }
        };
        self.update(v);
    }

    pub fn coerce_to_bytes(&mut self) {
        let v = {
            match *self.0.borrow() {
                ValueImpl::String(ref s) =>
                    Some(ValueImpl::Bytes(string_to_bytes(&s))),
                ValueImpl::Float(ref f) =>
                    Some(ValueImpl::Bytes(float_to_bytes(&f))),
                ValueImpl::Bytes(_) => None
            }
        };
        self.update(v);
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0.borrow() {
            ValueImpl::String(ref s) => write!(f,"{:?}",s),
            ValueImpl::Float(ref v) =>  write!(f,"{:?}",v),
            ValueImpl::Bytes(ref b) =>  write!(f,"{:?}",b)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Value;
    
    #[test]
    fn interconvert() {
        let v_f = Value::new_from_float(vec!{
            104.,101.,108.,108.,111.,32.,
            116.,225.,110.,97.,105.,115.,116.,101.
        });
        assert_eq!("[\"hello tánaiste\"]",format!("{:?}",v_f.as_string(|s| s.clone())));
        assert_eq!("[104.0, 101.0, 108.0, 108.0, 111.0, 32.0, 116.0, 225.0, 110.0, 97.0, 105.0, 115.0, 116.0, 101.0]",v_f.as_floats(|f| format!("{:?}",f)));
        let v_s = Value::new_from_string(vec!["hello tánaiste".to_string()]);
        assert_eq!("[\"hello tánaiste\"]",format!("{:?}",v_s));
        assert_eq!("[\"hello tánaiste\"]",format!("{:?}",v_s.as_string(|s| s.clone())));
        assert_eq!("[104.0, 101.0, 108.0, 108.0, 111.0, 32.0, 116.0, 225.0, 110.0, 97.0, 105.0, 115.0, 116.0, 101.0]",v_s.as_floats(|f| format!("{:?}",f)));
        let mut vc_s = Value::new_from_string(vec!["hello tánaiste".to_string()]);
        vc_s.coerce_to_float(); /* s->f */
        assert_eq!("[104.0, 101.0, 108.0, 108.0, 111.0, 32.0, 116.0, 225.0, 110.0, 97.0, 105.0, 115.0, 116.0, 101.0]",format!("{:?}",vc_s));
        vc_s.coerce_to_bytes(); /* f->b */
        assert_eq!("[104, 101, 108, 108, 111, 32, 116, 195, 161, 110, 97, 105, 115, 116, 101]",
            format!("{:?}",vc_s));        
        let mut vc_f = Value::new_from_float(vec!{
            104.,101.,108.,108.,111.,32.,
            116.,225.,110.,97.,105.,115.,116.,101.
        });
        vc_f.coerce_to_string(); /* f->s */
        assert_eq!("[\"hello tánaiste\"]",format!("{:?}",vc_f));        
        let by = vec!{
            104,101,108,108,111,32,116,195,161,110,97,105,115,116,101
        };
        vc_f.coerce_to_float(); /* b->f */
        assert_eq!("[104.0, 101.0, 108.0, 108.0, 111.0, 32.0, 116.0, 225.0, 110.0, 97.0, 105.0, 115.0, 116.0, 101.0]",format!("{:?}",vc_f));
        let mut vc_b = Value::new_from_bytes(by);
        vc_b.coerce_to_string(); /* b->s */
        assert_eq!("[\"hello tánaiste\"]",format!("{:?}",vc_b));
        vc_b.coerce_to_bytes(); /* s->b */
        assert_eq!("[104, 101, 108, 108, 111, 32, 116, 195, 161, 110, 97, 105, 115, 116, 101]",
            format!("{:?}",vc_b));
    }
}
