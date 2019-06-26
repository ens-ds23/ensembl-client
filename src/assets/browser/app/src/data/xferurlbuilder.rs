use std::collections::{ HashMap, HashSet };
use itertools::Itertools;

struct ChromBuilder {
    input: HashSet<(String,String)>,
    output: Vec<String>
}

impl ChromBuilder {
    fn build(input: &Vec<(String,String)>) -> Vec<String> {
        let mut obj = ChromBuilder { input: input.iter().cloned().collect(), output: Vec::new() };
        obj.run();
        obj.output
    }

    fn run(&mut self) {
        while self.input.len() > 0 {
            let mut wires : Vec<String> = self.all_wires().iter().cloned().collect();
            wires.sort();
            self.step(&wires[0]);
        }
    }

    fn all_wires(&self) -> HashSet<String> {
        self.input.iter().map(|(wire,leaf)| wire).cloned().collect()
    }

    fn all_leafs(&self, wire_wanted: &str) -> HashSet<String> {
        self.input.iter().filter(|(wire,leaf)| wire == wire_wanted)
            .map(|(wire,leaf)| leaf).cloned().collect()
    }

    fn wire_wants_all(&self, wire: &str, leafs: &HashSet<String>) -> bool {
        leafs.difference(&self.all_leafs(wire)).next().is_none()
    }

    fn all_wires_want_all(&self, leafs: &HashSet<String>) -> HashSet<String> {
        self.all_wires().iter().filter(|wire| self.wire_wants_all(wire,leafs))
            .cloned().collect()
    }

    fn build_part(&self, wires: &HashSet<String>, leafs: &HashSet<String>) -> String {
        format!("{}={}",wires.iter().sorted().join(","),leafs.iter().sorted().join(""))
    }

    fn step(&mut self, wire: &str) {
        let leafs = self.all_leafs(wire);
        let wires = self.all_wires_want_all(&leafs);
        self.output.push(self.build_part(&wires,&leafs));
        self.input.retain(move |(wire,leaf)| !(wires.contains(wire) && leafs.contains(leaf)));
    }
}

pub struct XferUrlBuilder {
    data: HashMap<String,Vec<(String,String)>>
}

impl XferUrlBuilder {
    pub fn new() -> XferUrlBuilder {
        XferUrlBuilder {
            data: HashMap::<String,Vec<(String,String)>>::new()
        }
    }
    
    pub fn add(&mut self, wire: &str, chrom: &str, leaf: &str) {
        let set = self.data.entry(chrom.to_string()).or_insert_with(||
            Vec::<(String,String)>::new()
        );
        set.push((wire.to_string(),leaf.to_string()));
    }
    
    fn emit_chrom(&self, values: &Vec<(String,String)>) -> String {
        let mut data = ChromBuilder::build(&values);
        data.sort();
        data.join(";")
    }
    
    pub fn emit(&self) -> String {
        let mut chroms = Vec::<(String,String)>::new();
        for (chrom,v) in &self.data {
            chroms.push((chrom.to_string(),self.emit_chrom(v)));
        }
        chroms.sort();
        let chroms : Vec<String> = chroms
                .iter()
                .map(|(chrom,value)| format!("{}:{}",chrom,value))
                .collect();
        chroms.iter().join("+")
    }
}
