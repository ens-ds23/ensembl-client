use std::iter::repeat;
use std::sync::{ Arc, Mutex };

use tánaiste::{
    Argument, Command, DataState, Instruction, ProcState, Signature,
    Value
};

fn not(data: &Vec<f64>) -> Vec<f64> {
    let mut out = Vec::<f64>::new();
    for v in data.iter() {
        out.push(if *v == 0. { 1. } else { 0. });
    }
    out
}

fn elide(data: &Vec<f64>, bools: &Vec<f64>, stride: &Vec<f64>) -> Vec<f64> {
    if bools.len() == 0 { return vec!{}; }
    let mut out = Vec::<f64>::new();
    let mut data_iter = data.iter();
    let mut bool_iter = bools.iter().cycle();
    let mut term = false;
    while !term {
        let b = *bool_iter.next().unwrap() != 0.;
        for _ in 0..stride[0] as usize {
            match data_iter.next() {
                Some(v) => { if b { out.push(*v); } }
                None => { term = true; break; }
            }
        }
    }
    out
}

fn pick(source: &Vec<f64>, palette: &Vec<f64>, stride: &Vec<f64>) -> Vec<f64> {
    let mut out = Vec::<f64>::new();
    let stride = stride[0] as usize;
    for s in source.iter() {
        for i in 0..stride {
            let offset = *s as usize*stride+i;
            if offset >= palette.len() {
                console!("ERROR IN PICK source={:?} palette={:?} stride={:?} offset={:?}",source,palette,stride,offset);
            }
            out.push(palette[offset]);
        }
    }
    out
}

fn picks(source: &Vec<f64>, palette: &Vec<String>, stride: &Vec<f64>) -> Vec<String> {
    let mut out = Vec::<String>::new();
    let stride = stride[0] as usize;
    for s in source.iter() {
        for i in 0..stride {
            let offset = *s as usize*stride+i;
            if offset >= palette.len() {
                console!("ERROR IN PICK source={:?} palette={:?} stride={:?} offset={:?}",source,palette,stride,offset);
            }
            out.push(palette[offset].clone());
        }
    }
    out
}

fn index(data: &Vec<f64>, palette: &Vec<f64>) -> Vec<f64> {
    let mut out = Vec::<f64>::new();
    let len = palette.len();
    for v in data.iter() {
        out.push(palette.iter()
                    .position(|x| x == v)
                    .unwrap_or(len) 
                    as f64
        );
    }
    out
}

fn runs(starts: &Vec<f64>, lens: &Vec<f64>) -> Vec<f64> {
    let mut out = Vec::<f64>::new();
    let mut len_iter = lens.iter().cycle();
    for v in starts.iter() {
        let len = len_iter.next().unwrap();
        for i in 0..*len as usize {
            out.push(v+i as f64);
        }
    }
    out
}

fn runs_of(lens: &Vec<f64>, values: &Vec<f64>) -> Vec<f64> {
    let mut out = Vec::<f64>::new();
    let mut val_iter = values.iter().cycle();
    for len in lens.iter() {
        let val = val_iter.next().unwrap();
        for _ in 0..*len as usize {
            out.push(*val);
        }
    }
    out
}

fn get(data: &Vec<f64>, indexes: &Vec<f64>) -> Vec<f64> {
    let mut out = Vec::<f64>::new();
    for i in indexes.iter() {
        let i = *i as usize % data.len();
        out.push(data[i]);
    }
    out
}

fn accn(data: &Vec<f64>, strides: &Vec<f64>) -> Vec<f64> {
    let mut out = Vec::<f64>::new();
    let mut acc = 0.;
    if strides.len() > 0 {
        let mut stride_iter = strides.iter().cycle();
        let mut reset_dist = 0;
        for v in data.iter() {
            if reset_dist == 0 {
                reset_dist = *stride_iter.next().unwrap() as u32;
                acc = 0.;
                if reset_dist == 0 { break; }
            }
            acc += v;
            reset_dist -= 1;
            out.push(acc);
        }
    } else {
        for v in data.iter() {
            acc += v;
            out.push(acc);
        }
    }
    out
}


// not #target, #source
pub struct Not(usize,usize);
// elide #target, #bools, #stride
pub struct Elide(usize,usize,usize);
// pick #tagret, #source, #palette, #stride
pub struct Pick(usize,usize,usize,usize);
// picks #tagret, #source, #palette, #stride
pub struct Picks(usize,usize,usize,usize);
// all #out, #start/end
pub struct All(usize,usize);
// index #out, #source, #palette
pub struct Index(usize,usize,usize);
// runs #out, #start, #len
pub struct Runs(usize,usize,usize);
// runsof #out, #len, #values
pub struct RunsOf(usize,usize,usize);
// get #out, #in, #index
pub struct Get(usize,usize,usize);
// merge #out, #select, #in0, in1, ...
pub struct Merge(usize,usize,Vec<usize>);
// accn #out, #parts, #strides
pub struct AccN(usize,usize,usize);
// length #len(in), #in [floats]
pub struct Length(usize,usize);
// lengths #len(string), #strings
pub struct Lengths(usize,usize);
// burst #out-strings, #in-strings
pub struct Burst(usize,usize);

impl Command for Elide {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.0).as_floats(|data| {
            regs.get(self.1).as_floats(|bools| {
                regs.get(self.2).as_floats(|stride| {
                    let data = elide(data,bools,stride);
                    regs.set(self.0,Value::new_from_float(data));
                });
            });
        });                   
        return 1;
    }
}

impl Command for Not {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|data| {
            regs.set(self.0,Value::new_from_float(not(data)));
        });                   
        return 1;
    }
}

impl Command for Pick {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|source| {
            regs.get(self.2).as_floats(|palette| {
                regs.get(self.3).as_floats(|stride| {
                    let data = pick(source,palette,stride);
                    regs.set(self.0,Value::new_from_float(data));
                });
            });
        });                   
        return 1;
    }
}

impl Command for Picks {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|source| {
            regs.get(self.2).as_string(|palette| {
                regs.get(self.3).as_floats(|stride| {
                    let data = picks(source,palette,stride);
                    regs.set(self.0,Value::new_from_string(data));
                });
            });
        });                   
        return 1;
    }
}

impl Command for Runs {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|start| {
            regs.get(self.2).as_floats(|len| {
                let data = runs(start,len);
                regs.set(self.0,Value::new_from_float(data));
            });
        });                   
        return 1;
    }
}

impl Command for RunsOf {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|lens| {
            regs.get(self.2).as_floats(|values| {
                let data = runs_of(lens,values);
                regs.set(self.0,Value::new_from_float(data));
            });
        });                   
        return 1;
    }
}

impl Command for All {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|start_end| {
            let v : Vec<f64> = (start_end[0] as i64..start_end[1] as i64)
                .map(|v| v as f64)
                .collect();
            regs.set(self.0,Value::new_from_float(v));
        });
        return 1;
    }
}

impl Command for Index {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|data| {
            regs.get(self.2).as_floats(|palette| {
                regs.set(self.0,Value::new_from_float(index(data,palette)))
            });
        });
        return 1;
    }
}

impl Command for Get {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|data| {
            regs.get(self.2).as_floats(|index| {
                regs.set(self.0,Value::new_from_float(get(data,index)))
            });
        });
        return 1;
    }
}

impl Command for Merge {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|data| {
            let mut out = Vec::<f64>::new();
            let mut index : Vec<usize> = repeat(0).take(self.2.len()).collect();
            for x in data.iter() {
                let reg_idx = (*x as usize) % self.2.len();
                let reg_num = self.2[reg_idx];
                let v = regs.get(reg_num).as_floats(|values| {
                    values[index[reg_idx] % values.len()]
                });
                index[reg_idx] += 1;
                out.push(v);
            }
            regs.set(self.0,Value::new_from_float(out));
        });
        return 1;
    }
}

impl Command for AccN {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|data| {
            regs.get(self.2).as_floats(|strides| {
                regs.set(self.0,Value::new_from_float(accn(data,strides)));
            });
        });
        return 1;
    }

}

impl Command for Length {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_floats(|data| {
            regs.set(self.0,Value::new_from_float(vec!{ data.len() as f64 }));
        });
        return 1;
    }
}

impl Command for Lengths {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_string(|strings| {
            let lengths = strings.iter().map(|x| x.len() as f64).collect();
            regs.set(self.0,Value::new_from_float(lengths));
        });
        return 1;
    }
}

impl Command for Burst {
    fn execute(&self, rt: &mut DataState, _proc: Arc<Mutex<ProcState>>) -> i64 {
        let regs = rt.registers();
        regs.get(self.1).as_string(|strings| {
            let burst : Vec<Vec<String>> = strings.iter().map(|x|
                x.chars().map(|x| x.to_string()).collect()
            ).collect();
            let burst = burst.iter().cloned().flatten().collect();
            regs.set(self.0,Value::new_from_string(burst));
        });
        return 1;
    }
}

pub struct ElideI();
pub struct NotI();
pub struct PickI();
pub struct PicksI();
pub struct AllI();
pub struct IndexI();
pub struct RunsI();
pub struct RunsOfI();
pub struct GetI();
pub struct MergeI();
pub struct AccNI();
pub struct LengthI();
pub struct LengthsI();
pub struct BurstI();

impl Instruction for ElideI {
    fn signature(&self) -> Signature { Signature::new("elide","rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Elide(args[0].reg(),args[1].reg(),args[2].reg()))
    }
}

impl Instruction for NotI {
    fn signature(&self) -> Signature { Signature::new("not","rr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Not(args[0].reg(),args[1].reg()))
    }
}

impl Instruction for PickI {
    fn signature(&self) -> Signature { Signature::new("pick","rrrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Pick(args[0].reg(),args[1].reg(),args[2].reg(),args[3].reg()))
    }
}

impl Instruction for PicksI {
    fn signature(&self) -> Signature { Signature::new("picks","rrrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Picks(args[0].reg(),args[1].reg(),args[2].reg(),args[3].reg()))
    }
}

impl Instruction for AllI {
    fn signature(&self) -> Signature { Signature::new("all","rr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(All(args[0].reg(),args[1].reg()))
    }
}

impl Instruction for IndexI {
    fn signature(&self) -> Signature { Signature::new("index","rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Index(args[0].reg(),args[1].reg(),args[2].reg()))
    }
}

impl Instruction for RunsI {
    fn signature(&self) -> Signature { Signature::new("runs","rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Runs(args[0].reg(),args[1].reg(),args[2].reg()))
    }
}

impl Instruction for RunsOfI {
    fn signature(&self) -> Signature { Signature::new("runsof","rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(RunsOf(args[0].reg(),args[1].reg(),args[2].reg()))
    }
}

impl Instruction for GetI {
    fn signature(&self) -> Signature { Signature::new("get","rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Get(args[0].reg(),args[1].reg(),args[2].reg()))
    }
}

impl Instruction for MergeI {
    fn signature(&self) -> Signature { Signature::new("merge","rrr+") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        let mut rest = args.clone();
        rest.remove(0);
        rest.remove(0);
        let rest = rest.iter().map(|x| x.reg()).collect();
        Box::new(Merge(args[0].reg(),args[1].reg(),rest))
    }
}

impl Instruction for AccNI {
    fn signature(&self) -> Signature { Signature::new("accn","rrr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(AccN(args[0].reg(),args[1].reg(),args[2].reg()))
    }
}

impl Instruction for LengthI {
    fn signature(&self) -> Signature { Signature::new("length","rr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Length(args[0].reg(),args[1].reg()))
    }
}

impl Instruction for LengthsI {
    fn signature(&self) -> Signature { Signature::new("lengths","rr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Lengths(args[0].reg(),args[1].reg()))
    }
}

impl Instruction for BurstI {
    fn signature(&self) -> Signature { Signature::new("burst","rr") }
    fn build(&self, args: &Vec<Argument>) -> Box<Command> {
        Box::new(Burst(args[0].reg(),args[1].reg()))
    }
}
