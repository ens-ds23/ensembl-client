pub trait Input {
    fn to_f32(&self, _dest: &mut Vec<f32>) {}
}

#[derive(Clone,Copy,Debug,Hash,PartialEq,Eq)]
pub struct DataGroupIndex(usize);

struct DataGroupState {
    batch: DataBatch,
    size: i32
}

#[derive(Clone,Copy,Debug)]
pub struct DataBatch(usize,u32);

impl DataBatch {
    pub fn group(&self) -> DataGroupIndex { DataGroupIndex(self.0) }
    pub fn id(&self) -> u32 { self.1 }
}

struct DataBatchSource {
    max_batch: u32    
}

impl DataBatchSource {
    fn new() -> DataBatchSource {
        DataBatchSource { max_batch: 0 }
    }
    
    fn next(&mut self, g: &DataGroupIndex) -> DataBatch {
        self.max_batch += 1;
        DataBatch(g.0,self.max_batch)
    }
    
    fn clear(&mut self) {
        self.max_batch = 0;
    }
}

/* BatchManager manages the running of multiple instances of a WebGL
 * program due to data-volume or different values for uniforms.
 * 
 * A DataGroupIndex represents a set of runs with particular parameter
 * settings. A DataBatch is an indivdual run within a DataGroupIndex.
 * 
 * A property with particular requirements for a uniform creates a new
 * DataGroupIndex at the time it is set up. A DataBatch is created, or an
 * existing one returned. Whether data is partitioned according to
 * batch (ie whether batches are used) is left to the individual program
 * objects. However both ObjectAttrib and ObjectUniform do implelement
 * them so, in parctice their use can be relied upon.
 * 
 * Properties which use groups implement ShapeContext which ensures they
 * are run prior to adding shapes. They create groups and populate
 * objects with the values of their uniforms before any shape is 
 * created. If no data goes into these groups, they are unused. Some,
 * such as spot values, have values known as part of the composition.
 * 
 * Program::draw is called just once per program. This iterates through
 * the batches and calls Object::execute on each.
 */

pub struct BatchManager {
    batch_source: DataBatchSource,
    max_group: u32,
    batches: Vec<DataBatch>,
    groups: Vec<DataGroupState>,
}

const BATCH_LIMIT : u32 = 65535;

impl BatchManager {
    pub fn new() -> BatchManager {
        BatchManager {
            batch_source: DataBatchSource::new(),
            max_group: 0,
            batches: Vec::<DataBatch>::new(),
            groups: Vec::<DataGroupState>::new(),
        }
    }
    
    pub fn reset(&mut self) {
        self.batch_source.clear();
        self.batches.clear();
        let len = self.groups.len();
        self.max_group = 0;
        self.groups.clear();
        for _ in 0..len {
            self.new_group();
        }
    }
    
    pub fn batches(&self) -> &Vec<DataBatch> { &self.batches }
        
    pub fn new_group(&mut self) -> DataGroupIndex {
        let g = DataGroupIndex(self.groups.len());        
        let new = self.batch_source.next(&g);
        self.batches.push(new.clone());
        self.groups.push(DataGroupState {
            batch: new.clone(),
            size: 0
        });
        g
    }
    
    fn maybe_new_batch(&mut self, g: &DataGroupIndex, points: u16) {
        if {
            let group = &self.groups[g.0];
            /* Create if full */
            (group.size as u32) + (points as u32) > BATCH_LIMIT
        } {
            let new = self.batch_source.next(&g);
            self.batches.push(new.clone());            
            self.groups[g.0] = DataGroupState {
                batch: new,
                size: 0
            };
        }
    }
    
    pub fn get_batch(&mut self, g: &DataGroupIndex, points: u16) -> DataBatch {
        self.maybe_new_batch(g,points);
        self.groups[g.0].size += points as i32;
        self.groups[g.0].batch
    }
}
