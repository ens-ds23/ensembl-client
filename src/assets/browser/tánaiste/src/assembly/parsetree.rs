use std::fmt;

use core::Value;
use super::escapes::string_escape;

#[derive(Clone,Debug)]
pub enum Argument {
    Reg(usize),
    Str(Vec<String>),
    Floats(Vec<f64>)
}

impl Argument {
    pub fn reg(&self) -> usize {
        match self {
            Argument::Reg(r) => *r,
            _ => panic!(format!("not a register {:?}",self))
        }
    }
    
    pub fn value(&self) -> Value {
        match self {
            Argument::Str(s) => Value::new_from_string(s.to_vec()),
            Argument::Floats(f) => Value::new_from_float(f.to_vec()),
            _ => panic!(format!("not a value {:?}",self))            
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum SimpleArgumentType {
    Reg,
    Str,
    Floats,
    Const
}

#[derive(Clone,Debug,PartialEq)]
pub enum ArgumentType {
    One(SimpleArgumentType),
    Many(SimpleArgumentType)
}

impl SimpleArgumentType {
    fn from_sig(c: char) -> Option<SimpleArgumentType> {
        match c {
            'r' => Some(SimpleArgumentType::Reg),
            's' => Some(SimpleArgumentType::Str),
            'f' => Some(SimpleArgumentType::Floats),
            'c' => Some(SimpleArgumentType::Const),
            _ => None
        }
    }
}

pub struct Signature(pub String,pub Vec<ArgumentType>);

impl Signature {
    pub fn new(name: &str, args_s: &str) -> Signature {
        let mut args_in = args_s.to_string();
        let mut args = Vec::<ArgumentType>::new();
        let mut more = None;
        if args_in.ends_with("+") {
            more = args_in.chars().rev().nth(1);
            let b = args_in.len()-2;
            args_in.split_off(b);
        }
        for c in args_in.chars() {
            let v = ArgumentType::One(SimpleArgumentType::from_sig(c).unwrap());
            args.push(v);
        }
        if let Some(c) = more {
            let v = ArgumentType::Many(SimpleArgumentType::from_sig(c).unwrap());
            args.push(v);
        }        
        Signature(name.to_string(),args)
    }
}

// TODO string escapes parse+serial
impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Argument::Reg(r) =>
                write!(f,"#{}",r),
            Argument::Str(s) => {
                let s : Vec<String> = s.iter().map(|x| 
                    format!("\"{}\"",string_escape(x,false))).collect();
                write!(f,"{{{}}}",s.join(", "))
            },
            Argument::Floats(ff) => {
                let mut ff_s = Vec::<String>::new();
                for f in ff {
                    ff_s.push(format!("{}",f));
                }
                write!(f,"[{}]",ff_s.join(", "))
            }
        }
    }
}

#[derive(Clone,Debug)]
pub enum Statement {
    Instruction(String,Vec<Argument>),
    Label(String)
}

#[derive(Clone,Debug)]
pub struct SourceCode {
    pub statements: Vec<Statement>
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Instruction(code,args) => {
                let mut args_s = Vec::<String>::new();
                for arg in args {
                    args_s.push(format!("{}",arg));
                }
                write!(f,"    {} {}\n",code,args_s.join(", "))
            },
            Statement::Label(name) =>
                write!(f,".{}:\n",name)
        }
    }
}

impl fmt::Display for SourceCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for s in self.statements.iter() {
            write!(f,"{}",s).ok();
        }
        Ok(())
    }
}
