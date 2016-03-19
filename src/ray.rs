extern crate nalgebra as na;

use na::{Vec3};

pub struct Ray {
    origin: Vec3<f64>,
    direction: Vec3<f64>,
}

impl Ray {
    fn getRayOrigin(&self) -> Vec3<f64> { self.origin }
    fn getRayDirection(&self) -> Vec3<f64> { self.direction }

    fn new(o: Vec3<f64>, d: Vec3<f64>) -> Self {
        Ray {
            origin: o,
            direction: d,
        }
    }

    fn default(&self) -> Self {
        Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0))
    }
}
