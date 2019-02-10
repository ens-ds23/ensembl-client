use super::lexer::{ Lexer, Token };
use super::parsetree::{ Statement, Argument, SourceCode };

struct Parse<'input> {
    error: Option<(usize,String,usize)>,
    stream: Box<Iterator<Item=(usize,Token,usize)> + 'input>,
    extra: Option<(usize,Token,usize)>,
    pos_got: (usize,usize)
}

impl<'input> Parse<'input> {
    fn new(input: &'input str) -> Parse<'input> {
        Parse {
            stream: Box::new(Lexer::<'input>::new(input).into_iter()),
            extra: None,
            error: None,
            pos_got: (0,0)
        }
    }
    
    fn next_tok(&mut self) -> (usize,Token,usize) {
        if let Some(t) = self.extra.take() {
            t
        } else {
            if let Some(t) = self.stream.next() {
                if let Token::Error(ref e) = t.1 {
                    self.set_error(e.to_string());
                }
                t
            } else {
                (0,Token::EOF,0)
            }
        }
    }

    fn next(&mut self) -> Token {
        let t = self.next_tok();
        self.pos_got = (t.0,t.2);
        t.1
    }

    fn unget(&mut self, t: Token) {
        self.extra = Some((self.pos_got.0,t,self.pos_got.1));
    }

    fn set_error(&mut self, e: String) {
        if self.error.is_none() {
            self.error = Some((self.pos_got.0,e,self.pos_got.1));
        }
    }

    fn parse_floats(&mut self) -> Vec<f64> {
        let mut out = Vec::<f64>::new();
        loop {
            let t = self.next();
            if let Token::Num(v) = t {
                out.push(v);
            } else {
                self.unget(t);
                break;
            }
            let c = self.next();
            if c != Token::Chr(',') {
                self.unget(c);
                break;
            }            
        }
        let c = self.next();
        if c != Token::Chr(']') {
            self.set_error(format!("Bad token {:?}",c));
        }
        out
    }

    fn parse_arguments(&mut self) -> Vec<Argument> {
        let mut out = Vec::<Argument>::new();
        loop {
            let t = self.next();
            if let Token::Reg(r) = t {
                out.push(Argument::Reg(r));
            } else if let Token::Chr('[') = t {
                out.push(Argument::Floats(self.parse_floats()));
            } else if let Token::Str(s) = t {
                out.push(Argument::Str(s));
            } else {
                self.unget(t);
                break;
            }
            let c = self.next();
            if c != Token::Chr(',') {
                self.unget(c);
                break;
            }
        }
        out
    }

    fn parse_instruction(&mut self) -> Statement {
        let ins_tok = self.next();
        if let Token::Code(ins) = ins_tok {
            Statement::Instruction(ins,self.parse_arguments())
        } else {
            self.set_error(format!("Bad instruction {:?}",ins_tok));
            Statement::Label("".to_string())
        }
    }

    fn parse_statement(&mut self) -> Statement {
        let t = self.next();
        if let Token::Id(v) = t {
            let t = self.next();
            if t != Token::Chr(':') {
                self.set_error("Bad token, missing :".to_string());
            }
            Statement::Label(v)
        } else {
            self.unget(t);
            self.parse_instruction()
        }
    }
    
    fn parse_program(&mut self) -> Result<SourceCode,String> {
        let mut out = Vec::<Statement>::new();
        loop {
            let t = self.next();
            if t == Token::EOF { break; }
            self.unget(t);
            out.push(self.parse_statement());
        }
        if let Some((a,ref error,b)) = self.error {
            Err(format!("{} at bytes {}-{}",error.to_string(),a,b).to_string())
        } else {
            Ok(SourceCode { statements: out })
        }
    }
}

pub fn parse_source(prog: &str) -> Result<SourceCode,String> {
    let mut p = Parse::new(prog);
    p.parse_program()
}

const P1 : &str = r#"
.hello:
    world #1,#2,[1,2,3.5]
    earth "tánaiste","\"\303\241\n"
"#;

const P1CHK : &str = r#".hello:
    world #1, #2, [1, 2, 3.5]
    earth "tánaiste", "\"á\n"
"#;

#[test]
fn parser() {
    let out = parse_source(P1).ok().unwrap();
    assert_eq!(format!("{}",out),P1CHK);
}
