use std::fmt::Debug;
use std::ops::{ Add, Sub, Mul, Div, Neg };
use drivers::webgl::program::Input;
use types::{ Dot, Edge, AxisSense, Corner, CLeaf };

/***** Rect types *****/

#[derive(Clone,Copy,Debug)]
pub struct Rect<T: Copy + Clone + Debug, U: Copy + Clone + Debug>(Dot<T,U>, Dot<T,U>);

pub type RFraction = Rect<f32,f32>;
pub type RLeaf = Rect<f32,i32>;
pub type RPixel = Rect<i32,i32>;

pub fn area<T: Clone+Copy+Debug,U: Clone+Copy+Debug>(x: Dot<T,U>, y: Dot<T,U>) -> Rect<T,U> {
    Rect(x,y)
}

pub fn area_size<T: Clone+Copy+Debug + Add<T,Output=T>,
                 U: Clone+Copy+Debug + Add<U,Output=U>>(x: Dot<T,U>, y: Dot<T,U>) -> Rect<T,U> {
    Rect(x,x+y)
}

pub fn area_centred<T: Clone+Copy+Debug + Add<T,Output=T> + 
                       Div<f32,Output=T> + Neg<Output=T>,
                    U: Clone+Copy+Debug + Add<U,Output=U> + 
                       Div<f32,Output=U> + Neg<Output=U>>(x: Dot<T,U>, y: Dot<T,U>) -> Rect<T,U> {
    Rect(x,x+y) + Dot(-y.0/2.,-y.1/2.)
}


/*** impls for area types ***/

impl<T: Clone+Copy+Debug, U: Clone+Copy+Debug> Rect<T,U> {
    pub fn x_edge(&self, x1: AxisSense, x2: AxisSense) -> Rect<Edge<T>,U> {
        Rect(self.0.x_edge(x1),self.1.x_edge(x2))
    }

    pub fn y_edge(&self, x1: AxisSense, x2: AxisSense) -> Rect<T,Edge<U>> {
        Rect(self.0.y_edge(x1),self.1.y_edge(x2))
    }
}

impl<T: Clone+Copy+Debug, U: Clone+Copy+Debug> Rect<Edge<T>,Edge<U>> {
    pub fn corners(&self) -> Rect<AxisSense,AxisSense> {
        Dot((self.0).corner(),(self.1).corner()).into()
    }
    
    pub fn quantity(&self) -> Rect<T,U> {        
        Rect((self.0).quantity(),(self.1).quantity())
    }
}

impl<T: Copy+Clone+Debug,
     U: Copy+Clone+Debug> Rect<T,U> {
    pub fn flip_d<A: Copy+Clone+Debug + PartialOrd,
                  B: Copy+Clone+Debug + PartialOrd>(&self, xs: Dot<Edge<A>,Edge<B>>) -> Rect<T,U> {
        let mut out = *self;
        if let AxisSense::Min = (xs.0).corner() { // x
            out = Rect(Dot((out.1).0,(out.0).1),
                       Dot((out.0).0,(out.1).1));
        }
        if let AxisSense::Min = (xs.1).corner() { // y
            out = Rect(Dot((out.0).0,(out.1).1),
                       Dot((out.1).0,(out.0).1));
        }
        out
    }
}

impl<T: Copy+Clone+Debug + Sub<T,Output=T> + Add<T,Output=T>,
     U: Copy+Clone+Debug + Sub<U,Output=U> + Add<U,Output=U>> Rect<T,U> {    
    pub fn flip_area(&self, d: Dot<T,U>, c: Corner) -> Dot<T,U> {
        Dot(
            match c.0 {
                AxisSense::Min => (self.1).0 - (d.0 - (self.0).0),
                AxisSense::Max => d.0
            },
            match c.1 {
                AxisSense::Min => (self.1).1 - (d.1 - (self.0).1),
                AxisSense::Max => d.1
            },
        )
    }
}

impl<T: Copy+Clone+Debug,
     U: Copy+Clone+Debug> Rect<T,U> {
    pub fn flip_r<A: Copy+Clone+Debug + PartialOrd,
                  B: Copy+Clone+Debug + PartialOrd>(&self, xs: Rect<Edge<A>,Edge<B>>) -> Rect<T,U> {
        let mut out = *self;
        if Dot((xs.0).0,(xs.1).0).is_backward() { // x
            out = Rect(Dot((out.1).0,(out.0).1),
                       Dot((out.0).0,(out.1).1));
        }
        if Dot((xs.0).1,(xs.1).1).is_backward() { // y
            out = Rect(Dot((out.0).0,(out.1).1),
                       Dot((out.1).0,(out.0).1));
        }
        out
    }
    pub fn offset(&self) -> Dot<T,U> { self.0 }
    pub fn far_offset(&self) -> Dot<T,U> { self.1 }
}

impl<T: Copy+Clone+Debug + Sub<T,Output=T>,
     U: Copy+Clone+Debug + Sub<U,Output=U>> Rect<T,U> {

    pub fn size(&self) -> Dot<T,U> { self.1-self.0 }

    pub fn at_origin(self) -> Rect<T,U> {
        Rect(self.0-self.0,self.1-self.0)
    }
}

impl<T: Copy+Clone+Debug + Sub<T,Output=T> + Add<T,Output=T>,
     U: Copy+Clone+Debug + Sub<U,Output=U> + Add<U,Output=U>> Rect<T,U> {
    pub fn inset(self, amt: Rect<T,U>) -> Rect<T,U> {
        Rect(self.0+amt.0,self.1-amt.1)
    }
}

impl<T: Copy+Clone+Debug + From<u8> + Sub<T,Output=T>,
     U: Copy+Clone+Debug + From<u8> + Sub<U,Output=U>> Rect<T,U> {
    pub fn area(&self) -> Dot<T,U> { self.1 - self.0 }
}

impl<T: Copy + Clone + Debug, U: Copy + Clone + Debug> Rect<T,U> {
    pub fn rectangle(&self) -> [Dot<T,U>;4] {
        [
            self.0,
            Dot((self.0).0, (self.1).1), 
            self.1,
            Dot((self.1).0, (self.0).1)
        ]
    }
}

impl<T : Copy + Clone + Debug + Into<f64>,
     U : Copy + Clone + Debug + Into<f64>> Rect<T,U> {    
    pub fn as_fraction(&self) -> RFraction {
        area(self.0.as_fraction(),self.1.as_fraction())
    }
}

impl<T : Copy + Clone + Debug + Into<f64>,
     U : Copy + Clone + Debug + Into<f64>> Input for Rect<T,U> {
    fn to_f32(&self, dest: &mut Vec<f32>) {
        for c in self.rectangle().iter() {
            dest.push(c.0.into() as f32);
            dest.push(c.1.into() as f32);
        }
    }    
}

/* Rect + Dot => offset */
impl<A: Copy+Clone+Debug + Add<C,Output=T>,
     B: Copy+Clone+Debug + Add<D,Output=U>,
     C: Copy+Clone+Debug,
     D: Copy+Clone+Debug,
     T: Copy+Clone+Debug,
     U: Copy+Clone+Debug>
        Add<Dot<C,D>> for Rect<A,B> {
    type Output = Rect<T,U>;
    
    fn add(self, other: Dot<C,D>) -> Rect<T,U> {
        Rect(self.0 + other, self.1 + other)
    }         
}

impl<A: Copy+Clone+Debug + Add<C,Output=T>,
     B: Copy+Clone+Debug + Add<D,Output=U>,
     C: Copy+Clone+Debug,
     D: Copy+Clone+Debug,
     T: Copy+Clone+Debug,
     U: Copy+Clone+Debug>
        Add<Rect<C,D>> for Dot<A,B> {
    type Output = Rect<T,U>;
    
    fn add(self, other: Rect<C,D>) -> Rect<T,U> {
        Rect(self + other.0, self + other.1)
    }
}

/* Rect * Dot => scale size and offset as given, into self */
impl<T : Copy + Clone + Debug + Mul<T, Output=T>,
     U : Copy + Clone + Debug + Mul<U, Output=U>> Mul<Dot<T,U>> for Rect<T,U> {
    type Output = Rect<T,U>;
    
    fn mul(self, other: Dot<T,U>) -> Rect<T,U> {
        Rect(self.0 * other, self.1 * other)
    }         
}

/* Rect / Dot => scale size and offset as given, into fraction */
impl<T : Copy + Clone + Debug + Div<T, Output=T> + Into<f32>,
     U : Copy + Clone + Debug + Div<U, Output=U> + Into<f32>> Div<Dot<T,U>> for Rect<T,U> {
    type Output = Rect<f32,f32>;
    
    fn div(self, other: Dot<T,U>) -> Rect<f32,f32> {
        Rect(self.0 / other, self.1 / other)
    }         
}

pub struct Bounds<T: Copy+Clone+Debug + PartialOrd,
                  U: Copy+Clone+Debug + PartialOrd>(Option<Rect<T,U>>);

impl<T: Copy+Clone+Debug + PartialOrd,
     U: Copy+Clone+Debug + PartialOrd> Bounds<T,U> {
    pub fn new() -> Bounds<T,U> { Bounds(None) }
    pub fn get(&self) -> Option<Rect<T,U>> { self.0 }
    
    pub fn add(&mut self, pt: Dot<T,U>) {
        if self.0.is_none() { self.0 = Some(area(pt,pt)); }
        self.0 = Some(
            Rect(self.0.unwrap().0.min(&pt),
                 self.0.unwrap().1.max(&pt))
        )
    }
}

#[derive(Clone,Copy,Debug)]
pub enum XPosition {
    Base(f64,i32,i32),
    Pixel(Edge<i32>,Edge<i32>)
}

#[derive(Clone,Copy,Debug)]
pub enum YPosition {
    Pixel(Edge<i32>,Edge<i32>),
    Page(i32,i32)
}

#[derive(Clone,Copy,Debug)]
pub enum Placement {
    Placed(XPosition,YPosition),
    Stretch(RLeaf)
}

impl Placement {
    pub fn add_bp(&self, bp: f64, bp_per_leaf: f64) -> Placement {
        match self {
            Placement::Placed(mut x,y) => {
                if let XPosition::Base(b,s,e) = x {
                    x = XPosition::Base(b*bp_per_leaf+bp,s,e)
                };
                Placement::Placed(x,*y)
            },
            Placement::Stretch(r) => {
                Placement::Stretch(*r * Dot(bp_per_leaf as f32,1) + Dot(bp as f32,0))
            }
        }
    }
}
