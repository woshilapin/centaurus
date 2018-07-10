use std::option::Option;

pub mod triangle;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        None
    }
}

pub struct Ray {
    origin: [f64; 3],
    direction: [f64; 3],
}

impl Ray {
    pub fn new(origin: [f64;3],direction:[f64;3]) -> Ray {
        Ray {
            origin,
            direction,
        }
    }
    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }
}


pub struct Intersection {
    position: [f64; 3],
}