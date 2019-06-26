use std::fmt;
use std::cmp::max;

use types::{ CPixel, cpixel };

/* An origin is the top left corner of an allocation or free space. The
 * extent of the space is not recorded and must be determined from
 * context.
 */

#[derive(Debug,Clone)]
struct Origin {
    pos: CPixel
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.pos.0, self.pos.1)
    }
}

impl Origin {
    /* Carve a space out of the top-left of another space, returning
     * 1-3 tranches. The first is always the desired space and has
     * width/height as requested. The second and third are the leftovers.
     */
    fn chop(&self, size: CPixel,
            total_width: i32, total_height: i32) -> Vec<Tranche> {
        let mut out = Vec::<Tranche>::new();
        out.push(Tranche {
            h: size.1,
            r: Origin { pos: self.pos }
        });
        if size.1 < total_height {
            out.push(Tranche {
                h: total_height-size.1,
                r: Origin { pos: cpixel(self.pos.0,self.pos.1 + size.1) }
            });
        }
        if size.0 < total_width {
            out.push(Tranche {
                h: size.1,
                r: Origin { pos: cpixel(self.pos.0 + size.0,self.pos.1) }
            });
        }
        out
    }
}

/* A tranche is an origin and a height. Where relevant, the width
 * implicitly extends to the end of the allocation unless otherwise
 * contextualised.
 */

struct Tranche {
    r: Origin,
    h: i32
}

impl Tranche {
    fn chop(&self, size: CPixel, max_width: i32) -> Vec<Tranche> {
        self.r.chop(size,max_width-self.r.pos.0,self.h)
    }
}

impl Clone for Tranche {
    fn clone(&self) -> Tranche { Tranche { h: self.h, r: self.r.clone() }}
}

/* A Height contains all the Tranches of a particular height. */

#[derive(Debug)]
struct Height {
    height: i32,
    spaces: Vec<Origin>
}

impl fmt::Display for Height {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let strings : Vec<String> = self.spaces.iter().map(|x| x.to_string()).collect();
        let csl : String = strings.join(", ");
        write!(f, "({},[{}])",self.height,csl)
    }
}

impl Height {
    fn new(height: i32) -> Height {
        Height {
            height,
            spaces: Vec::<Origin>::new()
        }
    }
    
    fn add(&mut self,tranche: Origin) {
        self.spaces.push(tranche);
    }

    /* Remove from the height a tranche of at least the given width.
     * It's the responsibility of the caller to chop it up and return
     * any fragments into the relevant Heights.
     */
    fn alloc(&mut self, width: i32, max_width: i32) -> Option<Tranche> {
        let mut target : Option<usize> = None;
        for (i,space) in self.spaces.iter().enumerate() {
            if space.pos.0 + width <= max_width {
                target = Some(i);
                break;
            }
        }
        match target {
            Some(i) => Some(Tranche {
                r: self.spaces.remove(i),
                h: self.height
            }),
            None => None
        }
    }
}

impl Clone for Height {
    fn clone(&self) -> Height {
        Height {
            height: self.height,
            spaces: self.spaces.to_vec()
        }
    }
}

/* Half is the main allocator struct for the top half and bottom half of
 * the allocation.
 */
 
#[derive(Debug)]
struct Half {
    width: i32,
    watermark: i32,
    spaces: Vec<Height>
}

impl fmt::Display for Half {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let strings : Vec<String> = self.spaces.iter().map(|x| x.to_string()).collect();
        let csl : String = strings.join(", ");
        write!(f, "({},{},[{}])",self.width,self.watermark,csl)
    }
}

impl Half {
    fn new(width: i32) -> Half {
        Half {
            width,
            watermark: 0,
            spaces: Vec::<Height>::new()
        }
    }
    
    fn add_space(&mut self, tranche: Tranche) {
        let h = tranche.h as usize;
        let len = self.spaces.len();
        if len < h {
            for h in len..h {
                self.spaces.push(Height::new((h+1) as i32));
            }
        }
        self.spaces[h-1].add(tranche.r);
    }
        
    fn alloc_space(&mut self, size: CPixel) -> Option<Origin> {
        let h = size.1 as usize;
        let len = self.spaces.len();
        if self.spaces.len() >= h {
            let mut frags : Option<Vec<Tranche>> = None;
            for i in h-1..len {
                let result = self.spaces[i].alloc(size.0,self.width);
                if let Some(tranche) = result {
                    frags = Some(tranche.chop(size,self.width));
                    break
                }
            }
            if let Some(frags) = frags {
                let len = frags.len();
                let out = frags[0].r.clone();
                if len > 1 {
                    for i in 1..len {
                        self.add_space(frags[i].clone());
                    }
                }
                return Some(out);
            }
        }
        None
    }
    
    fn alloc_watermark(&mut self,size: CPixel) -> Option<CPixel> {
        let tranche = Tranche {
            r: Origin { pos: cpixel(0,self.watermark) },
            h: size.1
        };
        self.watermark = self.watermark + size.1;
        self.add_space(tranche);
        let out = self.alloc_space(size);
        match out {
            Some(Origin { pos }) => Some(pos),
            None => {
                js! { console.log("Watermark alloc failed. Should be impossible"); };
                None
            }
        }
    }
    
    fn allocate(&mut self,size: CPixel) -> Option<CPixel> {
        match self.alloc_space(size) {
            Some(space) => Some(space.pos),
            None => self.alloc_watermark(size)
        }
    }
    
    fn get_height(&self) -> i32 { self.watermark }
    fn get_width(&self) -> i32 { self.width }
}

/* The allocator is the publicly available struct. It is implemented by
 * to Half-s. The only work here is managing tickets (to avoid borrower
 * hell in the public API) and to stitch the two halves together.
 */

struct TicketReq {
    size: CPixel
}

#[derive(Clone)]
struct TicketRes {
    pos: CPixel
}

pub struct Allocator {
    reqs: Vec<TicketReq>,
    res: Vec<TicketRes>,
    max_width: i32,
    area: i32,
    threshold: i32,
}

#[derive(Clone)]
pub struct Ticket {
    index: usize,
    size: CPixel
}

struct AllocatorImpl {
    big: Half,
    small: Half,
    threshold: i32,
}

fn pow2_i32(v: i32) -> i32 { (v as u32).next_power_of_two() as i32 }

impl AllocatorImpl {
    fn new(threshold: i32, max_width: i32, area: i32) -> AllocatorImpl {
        let size = pow2_i32(max(max_width,(area as f32).sqrt() as i32));
        AllocatorImpl {
            big: Half::new(size),
            small: Half::new(size),
            threshold
        }
    }
    
    fn allocate_one(&mut self, size: CPixel) -> (bool,CPixel) {
        if size.1 < (self.threshold / 2) {
            let pos = self.small.allocate(size).unwrap();
            (false,pos)
        } else {
            let height = ( size.1 + self.threshold - 1) / self.threshold;
            let pos = self.big.allocate(cpixel(size.0,height)).unwrap();
            (true,pos)
        }
    }
    
    fn width(&self) -> i32 { self.big.get_width() }
    fn split_point(&self) -> i32 { self.big.get_height() * self.threshold }
    fn total_height(&self) -> i32 { self.split_point() + self.small.get_height() }
}

impl Allocator {
    pub fn new(threshold: i32) -> Allocator {
        Allocator {
            reqs: Vec::<TicketReq>::new(),
            res:  Vec::<TicketRes>::new(),
            max_width: 0,
            area: 0,
            threshold,
        }
    }
    
    pub fn request(&mut self, size: CPixel) -> Ticket {
        self.max_width = max(size.0,self.max_width);
        self.area = self.area + size.0*size.1;
        let data = TicketReq { size };
        self.reqs.push(data);
        Ticket { index: self.reqs.len()-1, size }
    }

    pub fn allocate(&mut self) -> CPixel {
        let mut aimpl = AllocatorImpl::new(self.threshold,self.max_width,self.area);
        let mut res = Vec::<(bool,CPixel)>::new();
        for t in &self.reqs {
            res.push(aimpl.allocate_one(t.size));
        }
        let split = aimpl.split_point();
        for (big,pos) in res {
            let mut val = TicketRes { pos };
            if big {
                val.pos.1 *= self.threshold;
            } else {
                val.pos.1 += split;
            }
            self.res.push(val);
        }
        let height = pow2_i32(aimpl.total_height());
        cpixel(aimpl.width(), height)
    }
    
    pub fn position(&self,t : &Ticket) -> CPixel {
        self.res[t.index].clone().pos
    }
    
    pub fn size(&self,t : &Ticket) -> CPixel { t.size }
}

#[test]
fn half_test() {
    let mut alloc = Half::new(256);
    let input = [(100,6),(100,6),(100,6),(4,6),  (20,5), (50,1)];
    let check = [(0,0),  (100,0),(0,6),  (200,0),(100,6),(100,11)];
    for (i,(x,y)) in input.iter().enumerate() {
        let Dot(s,t) = alloc.allocate(cpixel(*x,*y)).unwrap();
        println!("({}x{}) -> ({},{}) want ({},{})",x,y,s,t,check[i].0,check[i].1);
        assert_eq!((s,t),check[i]);
    }
    assert_eq!(alloc.get_height(),12);
}

#[test]
fn full_test() {
    let mut ac = Allocator::new(16);
    let s = [(10,10),(100,1),(20,32),(2,12),(2,12),(50,2),(40,2),(120,2)];
    let c = [(0,0),   (0,48),(0,16), (10,0),(12,0),(0,49),(50,49),(0,51)];
    let mut t = Vec::<Ticket>::new();
    for (x,y) in &s {
        t.push(ac.request(cpixel(*x,*y)));
    }
    ac.allocate();
    for (i,t) in t.iter().enumerate() {
        let Dot(x,y) = ac.position(t);
        assert_eq!((x,y),c[i]);
    }
}
