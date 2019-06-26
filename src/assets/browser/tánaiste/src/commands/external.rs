use std::collections::HashMap;
use std::process;
use std::sync::{ Arc, Mutex };
use std::thread;

use assembly::{ Argument, Signature };
use core::{ Command, Instruction, Value };
use runtime::{ DataState, ProcState };
use util::ValueStore;

struct Result {
    exit_code: i32,
    stdout: String,
    stderr: String
}

lazy_static! {
    static ref RESULTS: Arc<Mutex<ValueStore<Option<Result>>>> =
        Arc::new(Mutex::new(ValueStore::<Option<Result>>::new()));
        
    static ref ID_MAP: Arc<Mutex<HashMap<(usize,usize,usize),usize>>> =
        Arc::new(Mutex::new(HashMap::<(usize,usize,usize),usize>::new()));
}

/****************************************
 * Synchronous external command: extern *
 ****************************************
 */

#[derive(Debug)]
pub struct External {
    code_reg: usize,
    out_reg: usize,
    command_reg: usize
}

impl External {
    pub fn new(code_reg: usize, out_reg: usize,
               command_reg: usize) -> Box<Command> {
        Box::new(External {
            code_reg, out_reg, command_reg
        })
    }
}

impl Command for External {
    fn execute(&self, data: &mut DataState, proc: Arc<Mutex<ProcState>>) -> i64 {
        let retry = data.continuations().get(1).as_floats(|f| 
            f.get(0).map(|s| *s)
        );
        if let Some(retry) = retry {
            let s = &mut RESULTS.lock().unwrap();
            let res = s.unstore(retry as usize-1).unwrap();
            data.registers().set(self.code_reg,Value::new_from_float(vec! {
                res.exit_code as f64
            }));
            if self.out_reg > 0 {
                data.registers().set(self.out_reg,
                    Value::new_from_string(vec![res.stdout,res.stderr]));
            }
            return 0;
        }
        let r_idx = RESULTS.lock().unwrap().store(None);
        data.set_again();
        data.continuations().set(1,Value::new_from_float(vec!{ (r_idx+1) as f64 })); 
        proc.lock().unwrap().sleep();
        let cmd_str = data.registers().get(self.command_reg).as_string(|s| s.clone());
        let res = RESULTS.clone();
        thread::spawn(move || {
            let c = process::Command::new("bash")
                        .arg("-c").arg(&cmd_str[0])
                        .stdout(process::Stdio::piped())
                        .stderr(process::Stdio::piped())
                        .spawn().ok().unwrap();
            let output = c.wait_with_output().ok().unwrap();
            let exit_code = output.status.code().unwrap_or(-1);
            let s = &mut res.lock().unwrap();
            s.replace(r_idx,Some(Result {
                exit_code,
                stdout: String::from_utf8_lossy(&output.stdout).clone().to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).clone().to_string(),
            }));
            proc.lock().unwrap().wake();
        });
        return 1000;
    }
}

pub struct ExternalI();

impl Instruction for ExternalI {
    fn signature(&self) -> Signature { Signature::new("extern","rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        External::new(args[0].reg(),args[1].reg(),args[2].reg())
    }
}

/*******************************************
 * Asynchronous external command: poextern *
 *******************************************
 */

#[derive(Debug)]
//                    fd    group cmd
pub struct PoExternal(usize,usize,usize);

impl Command for PoExternal {
    fn execute(&self, data: &mut DataState, proc: Arc<Mutex<ProcState>>) -> i64 {
        let mut polls = proc.lock().unwrap().polls().clone();
        /* allocate poll fd and assign */
        let g = data.registers().get(self.1).as_floats(|f| *f.get(0).unwrap_or(&0.));
        let id = polls.allocate(g);
        data.registers().set(self.0,Value::new_from_float(vec![ id ]));
        /* create a home for the output */
        let r_idx = RESULTS.lock().unwrap().store(None);
        let map_id = (proc.lock().unwrap().get_ipid(), g as usize, id as usize);
        ID_MAP.lock().unwrap().insert(map_id,r_idx);
        /* set command going */
        let cmd_str = data.registers().get(self.2).as_string(|s| s.clone());
        let res = RESULTS.clone();
        thread::spawn(move || {
            let c = process::Command::new("bash")
                        .arg("-c").arg(&cmd_str[0])
                        .stdout(process::Stdio::piped())
                        .stderr(process::Stdio::piped())
                        .spawn().ok().unwrap();
            let output = c.wait_with_output().ok().unwrap();
            let exit_code = output.status.code().unwrap_or(-1);
            let s = &mut res.lock().unwrap();
            s.replace(r_idx,Some(Result {
                exit_code,
                stdout: String::from_utf8_lossy(&output.stdout).clone().to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).clone().to_string(),
            }));
            polls.on_off(g,id,true);
        });
        return 1000;
    }
}

pub struct PoExternalI();

impl Instruction for PoExternalI {
    fn signature(&self) -> Signature { Signature::new("poextern","rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(PoExternal(args[0].reg(),args[1].reg(),args[2].reg()))
    }
}

#[derive(Debug)]
//                       code  out   group id
pub struct PoExternalRes(usize,usize,usize,usize);

impl Command for PoExternalRes {
    fn execute(&self, data: &mut DataState, proc: Arc<Mutex<ProcState>>) -> i64 {
        let g = data.registers().get(self.2).as_floats(|f| *f.get(0).unwrap_or(&0.));
        let id = data.registers().get(self.3).as_floats(|f| *f.get(0).unwrap_or(&0.));
        let map_id = (proc.lock().unwrap().get_ipid(), g as usize, id as usize);
        if let Some(r_idx) = ID_MAP.lock().unwrap().remove(&map_id) {
            let s = &mut RESULTS.lock().unwrap();
            let res = s.unstore(r_idx).unwrap();
            data.registers().set(self.0,Value::new_from_float(vec! {
                res.exit_code as f64
            }));
            if self.1 > 0 {
                data.registers().set(self.1,Value::new_from_string(vec![res.stdout,res.stderr]));
            }
        }
        return 100;
    }
}

pub struct PoExternalResI();

impl Instruction for PoExternalResI {
    fn signature(&self) -> Signature { Signature::new("poexternres","rrrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(PoExternalRes(args[0].reg(),args[1].reg(),args[2].reg(),
                               args[3].reg()))
    }
}

#[cfg(test)]
mod test {
    use test::{ command_run, TestContext };

    #[test]
    fn external() {
        let tc = TestContext::new();
        let mut r = command_run(&tc,"extern");
        assert_eq!("[0.0]",r.get_reg(1));
        assert_eq!("[1.0]",r.get_reg(2));
        assert_eq!("[\"/tmp/x\\n\", \"\"]",r.get_reg(4));
        assert_ne!("[\"\"]",r.get_reg(5));
        r.run();
    }

    #[test]
    fn po_external() {   
        let tc = TestContext::new();
        let mut r = command_run(&tc,"po-extern");
        assert_eq!("[\"hello\\n\", \"\"]",r.get_reg(8));
        assert_eq!("[0.0]",r.get_reg(9));
    }
}
