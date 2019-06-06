/* TODO deduplicate from tánaiste when we have enough for a crate of utilities */
use util::Smallest;

pub struct ValueStore<T> {
    smallest: Smallest,
    values: Vec<Option<T>>
}

impl<T> ValueStore<T> {
    pub fn new() -> ValueStore<T> {
        ValueStore {
            smallest: Smallest::new(),
            values: Vec::<Option<T>>::new(),
        }
    }
    
    pub fn store(&mut self, v: T) -> usize {
        let k = self.smallest.get();
        while k >= self.values.len() {
            self.values.push(None);
        }
        self.values[k] = Some(v);
        k
    }

    pub fn len(&self) -> usize { self.values.len() }

    pub fn every<'a>(&'a self) -> Box<Iterator<Item=usize>+'a> {
        Box::new(self.values.iter().enumerate()
                                .filter(|x| x.1.is_some())
                                .map(|x| x.0))
    }

    pub fn get(&self, k: usize) -> Option<&T> {
        self.values.get(k).and_then(|v| v.as_ref())
    }

    pub fn get_mut(&mut self, k: usize) -> Option<&mut T> {
        self.values.get_mut(k).and_then(|v| v.as_mut())
    }
    
    #[allow(unused)]    
    pub fn replace(&mut self, k: usize, v: T) {
        self.values[k] = Some(v);
    }
    
    #[allow(unused)]
    pub fn unstore(&mut self, k: usize) -> T {
        let out = self.values[k].take();
        self.smallest.put(k);
        out.unwrap()
    }
}
