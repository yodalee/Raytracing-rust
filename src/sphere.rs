extern crate nalgebra as na;
extern crate palette;

use na::{Vec3};
use palette::{Rgb, Rgba};

pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
    color: Rgba,
}

impl Sphere {
    pub fn getSphereCenter(&self) -> Vec3<f64> { self.center }
    pub fn getSphereRadius(&self) -> f64 { self.radius }
    pub fn getSphereColor(&self) -> Rgba { self.color }

    pub fn new(pos: Vec3<f64>, r: f64, color: Rgba) -> Self {
        Sphere {
            center: pos,
            radius: r,
            color: color,
        }
    }

    pub fn default() -> Self {
        Sphere::new(
            Vec3::new(0.0, 0.0, 0.0), 1.0, Rgba::new(0.5, 0.5, 0.5, 0.0)
        )
    }
}
