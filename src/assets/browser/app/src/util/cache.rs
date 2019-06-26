use std::hash::{ Hash, Hasher };
use std::collections::hash_map::DefaultHasher;
use std::marker::PhantomData;

/* 4-bit true LRU via bitmask: see lrugen.py */
const CACHE_LRU : [u8;64] = [
  3,  3,  9,  3,  3,  9,  3,  3,
  9,  9,  9,  0,  9,  9,  9,  0,
  9,  9,  9,  9,  1,  9,  1,  9,
  9,  9,  9,  9,  9,  9,  1,  0,
  2,  2,  9,  9,  9,  9,  9,  9,
  9,  2,  9,  0,  9,  9,  9,  9,
  2,  9,  9,  9,  1,  9,  9,  9,
  2,  2,  9,  0,  1,  9,  1,  0
];

const CACHE_USE_KEEP : [u8;4] = [
  0b110100, 0b101011, 0b011111, 0b111111
];

const CACHE_USE_SET : [u8;4] = [
  0b000000, 0b000001, 0b000110, 0b111000
];

pub struct Cache<K,V> {
    _k: PhantomData<K>,
    values: Vec<Option<(u64,V)>>,
    lru: Vec<u8>
}

impl<K,V> Cache<K,V> where K:Eq+Hash {
    pub fn new(size: usize) -> Cache<K,V> {
        if size % 4 != 0 {
            panic!("Cache size must be multiple of 4!");
        }
        let mut values = Vec::<Option<(u64,V)>>::new();
        values.reserve(size);
        for _ in 0..size {
            values.push(None);
        }
        let lru = std::iter::repeat(67).take(size/4).collect();
        Cache {
            _k: PhantomData,
            values,
            lru
        }
    }
    
    fn hash(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
    
    pub fn put(&mut self, key: &K, value: V) {
        let hash = self.hash(key);
        let h = hash as usize % (self.values.len()/4);
        let mut c = self.lru[h];
        let pos = if c > 63 {
            c -= 1;
            (66 - c) as usize
        } else {
            let p = CACHE_LRU[c as usize];
            if p == 9 { console!("Bad lookup ({:?})!",c); panic!("EEK"); }
            c &= CACHE_USE_KEEP[p as usize];
            c |= CACHE_USE_SET[p as usize];
            p as usize
        };
        self.lru[h] = c;
        self.values[h*4+pos] = Some((hash,value));
    }
    
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let hash = self.hash(key);
        let h = hash as usize % (self.values.len()/4);
        let mut c = self.lru[h];
        for i in 0..4 {
            if let Some((k,ref v)) = self.values[4*h+i] {
                if k == hash {
                    if c < 64 {
                        c &= CACHE_USE_KEEP[i as usize];
                        c |= CACHE_USE_SET[i as usize];
                    }
                    self.lru[h] = c;
                    return Some(v)
                }
            }
        }
        None
    }
}
