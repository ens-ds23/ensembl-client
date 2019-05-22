use std::cmp::{ Ordering, PartialEq, PartialOrd };
use std::fmt::Debug;
use std::ops::{ Add, Sub, Mul, Div, Neg };

use composit::Stage;
use program::Input;
use types::{
    Rect, Anchored, Axis, Edge, Corner, Anchor, Direction,
    RIGHT, LEFT, DOWN, UP
};

/***** Direction types *****/

#[derive(Clone,Copy,Debug)]
pub enum Units { Pixels, Bases, Screens }

impl Input for f32 {
    fn to_f32(&self, dest: &mut Vec<f32>) {
        dest.push(*self);
    }
}

impl<T: Clone+Copy+Debug + Neg<Output=T>,
     U: Clone+Copy+Debug + Neg<Output=U>> Dot<T,U> {
    pub fn flip(&self, c: &Corner) -> Dot<T,U> {
        Dot(c.0.flip(self.0), c.1.flip(self.1))
    }
}

impl<T: Copy+PartialEq,U: Copy+PartialEq> PartialEq for Dot<T,U> {
    fn eq(&self, other: &Dot<T,U>) -> bool {
        return self.0 == other.0 && self.1 == other.1
    }
}

impl Dot<f64,f64> {
    pub fn len(&self) -> f64 {
        (self.0*self.0 + self.1*self.1).sqrt()
    }
}

impl Dot<Anchor,Anchor> {
    pub fn to_middle(&self, r: Rect<f32,f32>) -> CFraction {
        let s = r.size() / cfraction(2.,2.);
        Dot(self.0.prop(),self.1.prop()) * s
    }
}

pub type EPixel = Dot<Edge<i32>,Edge<i32>>;

#[derive(Clone,Copy,Debug)]
pub struct Distance<T : Clone + Copy + Debug>(pub T,pub Units);

impl<T: Clone + Copy + Mul<f64,Output=T> + Div<f64,Output=T> + Debug> Distance<T> {
    pub fn convert(&self, target: Units, axis: Axis, stage: &Stage) -> Distance<T> {
        let Distance(quant,source) = self;
        let dims = stage.get_size();
        let (size,zoom) = match axis {
            Axis::Horiz => (dims.0 as f64,stage.get_screen_in_bp() as f64),
            Axis::Vert => (dims.1 as f64,1.0),
            Axis::Zoom => (1.,1.), // TODO
        };
        let quant = match source {
            Units::Pixels => match target {
                Units::Pixels => *quant,
                Units::Bases => *quant * zoom / size,
                Units::Screens => *quant / size
            },
            Units::Bases => match target {
                Units::Pixels => *quant / zoom * size,
                Units::Bases => *quant,
                Units::Screens => *quant / zoom
            },
            Units::Screens => match target {
                Units::Pixels => *quant * size,
                Units::Bases => *quant * zoom,
                Units::Screens => *quant
            },            
        };
        Distance(quant,target)
    }
}

#[derive(Clone,Copy,Debug)]
pub enum Move<T: Neg<Output=T> + Clone + Copy + Debug,
              U: Neg<Output=U> + Clone + Copy + Debug> {
    Up(Distance<U>),
    Down(Distance<U>),
    Left(Distance<T>),
    Right(Distance<T>)
}

impl<T: Neg<Output=T> + Clone+Copy+Debug + Mul<f64,Output=T> + 
        Div<f64,Output=T> + PartialOrd<f64>,
     U: Neg<Output=U> + Clone+Copy+Debug + Mul<f64,Output=U> + 
        Div<f64,Output=U> + PartialOrd<f64>> Move<T,U> {
         
    pub fn convert(&self, target: Units, stage: &Stage) -> Move<T,U> {
        match self {
            Move::Up(u) => Move::Up(u.convert(target,Axis::Vert, stage)),
            Move::Down(u) => Move::Down(u.convert(target,Axis::Vert, stage)),
            Move::Left(u) => Move::Left(u.convert(target,Axis::Horiz, stage)),
            Move::Right(u) => Move::Right(u.convert(target,Axis::Horiz, stage)),
        }
    }
    
    pub fn direction(&self) -> Direction {
        match self {
            Move::Up(_) => UP,
            Move::Down(_) => DOWN,
            Move::Left(_) => LEFT,
            Move::Right(_) => RIGHT
        }
    }

    pub fn true_direction(&self) -> Direction {
        match self {
            Move::Up(x) =>    if x.0 > 0. { UP } else { DOWN }
            Move::Down(x) =>  if x.0 > 0. { DOWN } else { UP }
            Move::Left(x) =>  if x.0 > 0. { LEFT } else { RIGHT }
            Move::Right(x) => if x.0 > 0. { RIGHT } else { LEFT }
        }
    }
}

impl<T: Clone + Copy + Debug + Add<T,Output=T> + Neg<Output=T>,
     U: Clone + Copy + Debug + Add<U,Output=U> + Neg<Output=U>> 
        Add<Move<T,U>> for Dot<T,U> {
    type Output = Dot<T,U>;
         
    fn add(self, other: Move<T,U>) -> Dot<T,U> {
        match other {
            Move::Up(Distance(y,_)) =>    Dot(self.0,      self.1+(-y)),
            Move::Down(Distance(y,_)) =>  Dot(self.0,      self.1+y),
            Move::Left(Distance(x,_)) =>  Dot(self.0+(-x), self.1),
            Move::Right(Distance(x,_)) => Dot(self.0+x,    self.1),
        }        
    }
}


/***** Dot types *****/

#[derive(Clone,Copy,Debug)]
pub struct Dot<T : Clone + Copy,
               U : Clone + Copy>(pub T, pub U);

pub type CFraction = Dot<f32,f32>;
pub fn cfraction(x: f32, y: f32) -> CFraction { Dot(x,y) }

pub type CDFraction = Dot<f64,f64>;
pub fn cdfraction(x: f64, y: f64) -> CDFraction { Dot(x,y) }


pub type CLeaf = Dot<f32,i32>;
pub fn cleaf(x: f32, y: i32) -> CLeaf { Dot(x,y) }

pub type CPixel = Dot<i32,i32>;
pub fn cpixel(x: i32, y: i32) -> CPixel { Dot(x,y) }

pub type APixel = Dot<Anchored<i32>,Anchored<i32>>;

/*** impls for dot types ***/

impl<T: Clone + Copy + Mul<T, Output=U>,U: Add<U, Output=U>> Dot<T,T> {
    pub fn abs_sq(&self) -> U {
        self.0*self.0+self.1*self.1
    }
}

impl<T : Clone + Copy + Into<f64>,
     U : Clone + Copy + Into<f64>> Dot<T,U> {    
    pub fn as_fraction(&self) -> CFraction {
        cfraction(self.0.into() as f32,self.1.into() as f32)
    }

    pub fn as_dfraction(&self) -> CDFraction {
        cdfraction(self.0.into() as f64,self.1.into() as f64)
    }
}

impl<T : Clone + Copy + Into<f64>,
     U : Clone + Copy + Into<f64>> Dot<Anchored<T>,Anchored<U>> {
    pub fn as_fraction(&self) -> Dot<Anchored<f32>,Anchored<f32>> {
        Dot(self.0.as_fraction(),self.1.as_fraction())
    }
}

fn pmin<'a,T: PartialOrd>(a: &'a T, b: &'a T) -> &'a T {
    if let Some(Ordering::Greater) = a.partial_cmp(b) { b } else { a }
}

fn pmax<'a,T: PartialOrd>(a: &'a T, b: &'a T) -> &'a T {
    if let Some(Ordering::Less) = a.partial_cmp(b) { b } else { a }
}

impl<T: Clone+Copy+Debug + PartialOrd,
     U: Clone+Copy+Debug + PartialOrd> Dot<T,U> {
    pub fn min(&self, other: &Dot<T,U>) -> Dot<T,U> {
        Dot(*pmin(&self.0,&other.0),*pmin(&self.1,&other.1))
    }

    pub fn max(&self, other: &Dot<T,U>) -> Dot<T,U> {
        Dot(*pmax(&self.0,&other.0),*pmax(&self.1,&other.1))
    }
}

impl<T : Clone + Copy + Into<f64>,
     U : Clone + Copy + Into<f64>> Input for Dot<T,U> {
    fn to_f32(&self, dest: &mut Vec<f32>) {
        let (a,b): (f64,f64) = (self.0.into(), self.1.into());
        dest.push(a as f32);
        dest.push(b as f32);
    }
}

/* Dot + Dot => add like vectors */
impl<A: Clone+Copy+Debug + Add<C,Output=T>,
     B: Clone+Copy+Debug + Add<D,Output=U>,
     C: Clone+Copy+Debug,
     D: Clone+Copy+Debug,
     T: Clone+Copy+Debug,
     U: Clone+Copy+Debug> Add<Dot<C,D>> for Dot<A,B> {
    type Output = Dot<T,U>;
    
    fn add(self, other: Dot<C,D>) -> Self::Output {
        Dot(self.0 + other.0, self.1+other.1)
    }
}

/* Dot - Dot => subtract like vectors */
impl<T : Clone+Copy + Sub<T, Output=T>,
     U : Clone+Copy + Sub<U, Output=U>> Sub for Dot<T,U> {
    type Output = Dot<T,U>;
    
    fn sub(self,other: Dot<T,U>) -> Dot<T,U> {
        Dot(self.0-other.0, self.1-other.1)
    }
}

impl<T : Clone+Copy + Neg<Output=T>,
     U : Clone+Copy + Neg<Output=U>> Neg for Dot<T,U> {
    type Output = Dot<T,U>;
    
    fn neg(self) -> Dot<T,U> {
        Dot(-self.0,-self.1)
    }
}

/* Dot * Dot => scale */
impl<T : Clone + Copy + Mul<T, Output=T>,
     U : Clone + Copy + Mul<U, Output=U>> Mul for Dot<T,U> {
    type Output = Dot<T,U>;
    
    fn mul(self,other: Dot<T,U>) -> Dot<T,U> {
        Dot(self.0*other.0, self.1*other.1)
    }
}

/* Dot/Dot => divide and force into fractional Dot */
impl<T: Clone + Copy + Div<T, Output=T> + Into<f32>,
     U: Clone + Copy + Div<U, Output=U> + Into<f32>> Div for Dot<T,U> {
    type Output = Dot<f32,f32>;
    
    fn div(self, other: Dot<T,U>) -> Dot<f32,f32> {
        Dot((self.0.into() as f64 / other.0.into() as f64) as f32,
            (self.1.into() as f64 / other.1.into() as f64) as f32)
    }
}

pub fn ddiv<T: Clone + Copy + Div<T, Output=T> + Into<f64>,
        U: Clone + Copy + Div<U, Output=U> + Into<f64>>(a: Dot<T,U>, b: Dot<T,U>) -> Dot<f64,f64> {
    Dot((a.0.into() as f64 / b.0.into() as f64) as f64,
        (a.1.into() as f64 / b.1.into() as f64) as f64)
}
