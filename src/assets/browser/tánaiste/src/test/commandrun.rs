use std::{ time, thread };

use assembly::assemble;
use core::{
    BinaryCode, InstructionSet, instruction_bundle_core, 
    instruction_bundle_native
};
use runtime::{ Process, PROCESS_CONFIG_DEFAULT };
use test::{ instruction_bundle_test, TEST_CODE, TestContext };

pub fn command_compile(what: &str, tc: &TestContext) -> BinaryCode {
    let is = InstructionSet::new(vec!{ 
        instruction_bundle_core(),
        instruction_bundle_native(),
        instruction_bundle_test(tc)
    });
    assemble(&is,&TEST_CODE[what]).expect("compile failed")
}

pub fn command_make(tc: &TestContext, what: &str) -> Process {
    let bin = command_compile(what,tc);
    bin.exec(None,None,&PROCESS_CONFIG_DEFAULT).ok().unwrap()
}

pub fn command_run(tc: &TestContext, what: &str) -> Process {
    let mut r = command_make(tc,what);
    r.run();
    while !r.halted() {
        r.run();
        thread::sleep(time::Duration::from_millis(100));
    }
    r
}
