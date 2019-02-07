use std::cmp::{ Ordering, PartialOrd };
use std::fmt::Debug;
use std::ops::{ Add, Neg };

use program::Input;
use types::{ Dot, area, Rect };

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Axis { Horiz, Vert }

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum AxisSense { Pos, Neg }

#[derive(Clone,Copy,Debug)]
pub struct Anchor(Option<AxisSense>);

impl Anchor {
    pub fn prop(&self) -> f32 {
        match self.0 {
            Some(AxisSense::Pos) =>  1.0,
            Some(AxisSense::Neg) => -1.0,
            None => 0.0
        }
    }
    
    pub fn flip(&self) -> Anchor {
        Anchor(match self.0 {
            Some(AxisSense::Pos) => Some(AxisSense::Neg),
            Some(AxisSense::Neg) => Some(AxisSense::Pos),
            None => None
        })
    }

    pub fn sense(self, s: AxisSense) -> Anchor {
        match s {
            AxisSense::Pos => self,
            AxisSense::Neg => self.flip()
        }
    }
}

pub type Anchors = Dot<Anchor,Anchor>;

impl Anchors {
    pub fn flip(&self, c: &Corner) -> Anchors {
        Dot(self.0.sense(c.0),self.1.sense(c.1))
    }
}

const AS : Anchor = Anchor(Some(AxisSense::Pos));
const AM : Anchor = Anchor(None);
const AE : Anchor = Anchor(Some(AxisSense::Neg));

pub const A_TOP        : Anchors = Dot(AM,AS);
pub const A_TOPLEFT    : Anchors = Dot(AS,AS);
pub const A_TOPRIGHT   : Anchors = Dot(AE,AS);
#[allow(unused)]
pub const A_BOTTOM     : Anchors = Dot(AM,AE);
pub const A_BOTTOMLEFT : Anchors = Dot(AS,AE);
pub const A_BOTTOMRIGHT: Anchors = Dot(AE,AE);
pub const A_LEFT       : Anchors = Dot(AS,AM);
pub const A_RIGHT      : Anchors = Dot(AE,AM);
pub const A_MIDDLE     : Anchors = Dot(AM,AM);

#[derive(PartialEq,Eq,Debug)]
pub struct Direction(pub Axis,pub AxisSense);

pub const LEFT : Direction = Direction(Axis::Horiz,AxisSense::Neg);
pub const RIGHT : Direction = Direction(Axis::Horiz,AxisSense::Pos);
pub const UP : Direction = Direction(Axis::Vert,AxisSense::Neg);
pub const DOWN : Direction = Direction(Axis::Vert,AxisSense::Pos);

#[derive(Clone,Copy,Debug)]
pub struct Corner(pub AxisSense, pub AxisSense);

pub const TOPLEFT    : Corner = Corner(AxisSense::Pos,AxisSense::Pos);
pub const TOPRIGHT   : Corner = Corner(AxisSense::Neg,AxisSense::Pos);
pub const BOTTOMLEFT : Corner = Corner(AxisSense::Pos,AxisSense::Neg);
pub const BOTTOMRIGHT: Corner = Corner(AxisSense::Neg,AxisSense::Neg);

impl From<AxisSense> for f32 {
    fn from(xs: AxisSense) -> f32 {
        let x : f64 = xs.into();
        x as f32
    }
}

impl From<AxisSense> for f64 {
    fn from(xs: AxisSense) -> f64 {
        match xs {
            AxisSense::Pos =>  1.0,
            AxisSense::Neg => -1.0
        }        
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Edge<T>(AxisSense,T);

#[derive(Clone,Copy,Debug)]
pub struct Anchored<T>(Anchor,T);

impl<T: Clone+Copy> Anchored<T> {
    pub fn prop(&self) -> f32 { self.0.prop() }
    pub fn flip(&self) -> Anchored<T> {
        Anchored(self.0.flip(),self.1)
    }

    pub fn sense<U>(&self, e: Edge<U>) -> Anchored<T> {
        Anchored(self.0.sense(e.0),self.1)
    }
}

impl AxisSense {
    pub fn flip<T: Clone+Copy+Debug + Neg<Output=T>>(&self, t: T) -> T {
        match self {
            AxisSense::Pos => t,
            AxisSense::Neg => -t
        }
    }
}

impl<T: Clone+Copy+Debug> Edge<T> {
    pub fn corner(self) -> AxisSense { self.0 }
    pub fn quantity(self) -> T { self.1 }
}

impl<T: Clone+Copy+Debug> Anchored<T> {
    pub fn corner(self) -> Option<AxisSense> { (self.0).0 }
    pub fn quantity(self) -> T { self.1 }
}

impl From<Dot<Corner,Corner>> for Rect<AxisSense,AxisSense> {
    fn from(c: Dot<Corner,Corner>) -> Rect<AxisSense,AxisSense> {
        area(Dot((c.0).0,(c.0).1),Dot((c.1).0,(c.1).1))
    }
}

impl Input for Corner {
    fn to_f32(&self, dest: &mut Vec<f32>) {
        let (a,b): (f32,f32) = (self.0.into(), self.1.into());
        dest.push(a);
        dest.push(b);
    }
}

impl<T: Clone+Copy+Debug, U: Clone+Copy+Debug> Dot<T,U> {
    pub fn x_edge(&self,xs: AxisSense) -> Dot<Edge<T>,U> {
        Dot(Edge(xs,self.0),self.1)
    }

    pub fn y_edge(&self,xs: AxisSense) -> Dot<T,Edge<U>> {
        Dot(self.0,Edge(xs,self.1))
    }

    pub fn x_anchor(&self,xs: Anchor) -> Dot<Anchored<T>,U> {
        Dot(Anchored(xs,self.0),self.1)
    }

    pub fn y_anchor(&self,xs: Anchor) -> Dot<T,Anchored<U>> {
        Dot(self.0,Anchored(xs,self.1))
    }
    
    pub fn anchor(&self, a: Anchors) -> Dot<Anchored<T>,Anchored<U>> {
        self.x_anchor(a.0).y_anchor(a.1)
    }
}

impl<T: Clone+Copy+Debug, U: Clone+Copy+Debug> Dot<Edge<T>,Edge<U>> {
    pub fn corner(&self) -> Corner {
        Corner((self.0).0,(self.1).0)
    }
    
    pub fn quantity(&self) -> Dot<T,U> {
        Dot((self.0).1,(self.1).1)
    }
}

impl<T: Clone+Copy+Debug, U: Clone+Copy+Debug> Dot<Anchored<T>,Anchored<U>> {
    pub fn corner(&self) -> Dot<Anchor,Anchor> {
        Dot((self.0).0,(self.1).0)
    }
    
    pub fn quantity(&self) -> Dot<T,U> {
        Dot((self.0).1,(self.1).1)
    }
    
    pub fn flip<A: Clone+Copy+Debug,
                B: Clone+Copy+Debug>(&self, f: Dot<Edge<A>,Edge<B>>) -> Dot<Anchored<T>,Anchored<U>> {
        Dot(self.0.sense(f.0),self.1.sense(f.1))
    }
}

impl<T : Clone + Copy + Into<f64>> Anchored<T> {
    pub fn as_fraction(&self) -> Anchored<f32> {
        Anchored(self.0,self.1.into() as f32)
    }
}

impl<T: Clone+Copy+Debug + PartialOrd> Dot<Edge<T>, Edge<T>> {    
    pub fn is_backward(&self) -> bool {
        match ((self.0).0,(self.1).0) {
            (AxisSense::Pos,AxisSense::Pos) =>
                ((self.0).1).partial_cmp(&(self.1).1).unwrap() == Ordering::Greater,
            (AxisSense::Neg,AxisSense::Neg) =>
                ((self.0).1).partial_cmp(&(self.1).1).unwrap() == Ordering::Less,
            (AxisSense::Neg,AxisSense::Pos) =>
                true,
            (AxisSense::Pos,AxisSense::Neg) =>
                false
        }
    }
}

pub fn cedge<T: Clone+Copy+Debug,U: Clone+Copy+Debug>
        (c: Corner, d: Dot<T,U>) -> Dot<Edge<T>,Edge<U>> {
    Dot(Edge(c.0,d.0),Edge(c.1,d.1))
}

impl<T: Clone+Copy+Debug+Add<T,Output=U>,
     U: Clone+Copy+Debug> Add<T> for Edge<T> {
    type Output = Edge<U>;
    
    fn add(self, other: T) -> Self::Output {
        Edge(self.0, self.1+other)
    }
    
}
