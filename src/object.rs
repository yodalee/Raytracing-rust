extern crate nalgebra as na;

use na::{Vec3};
use ray::{Ray};

pub trait Object {
    fn getNormalAt(&self, point: Vec3<f64>) -> Vec3<f64>;
    fn findIntersection(&self, ray: &Ray) -> f64;
}
