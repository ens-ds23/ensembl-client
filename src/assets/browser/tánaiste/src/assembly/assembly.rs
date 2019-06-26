use core::{ BinaryCode, InstructionSet };
use super::codegen::codegen;
use super::parser::parse_source;

pub fn assemble(is: &InstructionSet, code: &str) -> Result<BinaryCode,Vec<String>> {
    match parse_source(code) {
        Ok(src) => codegen(is,&src),
        Err(e) => Err(vec! { e })
    }
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::time;

    use core::{ BinaryCode, instruction_bundle_core, InstructionSet };
    use runtime::PROCESS_CONFIG_DEFAULT;
    use test::{ TEST_CODE, instruction_bundle_test, TestContext };
    use super::assemble;
    
    fn test_assemble(name: &str) -> Result<BinaryCode,Vec<String>> {
        let tc = TestContext::new();
        let is = InstructionSet::new(vec! {
            instruction_bundle_core(),
            instruction_bundle_test(&tc)
        });
        assemble(&is,&TEST_CODE[name])
    }

    #[test]
    fn smoke() {
        let bin = test_assemble("smoke").ok().unwrap();
        let mut proc = bin.exec(Some("start"),None,&PROCESS_CONFIG_DEFAULT).ok().unwrap();
        proc.run();
        while !proc.halted() {
            proc.run();
            thread::sleep(time::Duration::from_millis(100));
        }
        assert_eq!(proc.get_reg(1),"[\"hello, tánaiste!\"]");
    }

    #[test]
    fn parse_error_1() {
        let e = test_assemble("parse-error-1").err().unwrap();
        assert_eq!("Bad string: bad utf8: invalid utf-8 sequence of 1 bytes from index 7 at line 1",e.join("\n"));
    }

    #[test]
    fn parse_error_2() {
        let e = test_assemble("parse-error-2").err().unwrap();
        assert_eq!("Bad instruction Chr(\':\') at line 2",e.join("\n"));
    }

    #[test]
    fn inst_error_1() {
        let e = test_assemble("inst-error-1").err().unwrap();
        assert_eq!("Unknown command \'unknown\'",e.join("\n"));
    }

    #[test]
    fn inst_error_2() {
        let e = test_assemble("inst-error-2").err().unwrap();
        assert_eq!("Incorrect arguments [Str([\"hello\"])] vs [One(Reg)]",e.join("\n"));
    }
}
