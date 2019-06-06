/* TODO deduplicate from tánaiste when we have enough for a crate of utilities */
use std::collections::BinaryHeap;

pub struct Smallest {
    heap: BinaryHeap<i64>,
    next: i64
}

impl Smallest {
    pub fn new() -> Smallest {
        Smallest {
            heap: BinaryHeap::<i64>::new(),
            next: 0
        }
    }
    
    pub fn get(&mut self) -> usize {
        if self.heap.is_empty() {
            self.heap.push(-self.next);
            self.next += 1;
        }
        -self.heap.pop().unwrap() as usize
    }
    
    #[allow(unused)]
    pub fn put(&mut self, v: usize) {
        self.heap.push(-(v as i64))
    }    
}
