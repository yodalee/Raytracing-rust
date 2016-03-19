extern crate nalgebra as na;
extern crate palette;

use na::{Vec3};
use palette::{Rgb, Rgba};

pub struct Light {
    position: Vec3<f64>,
    color: Rgba,
}

impl Light {
    pub fn getLightPosition(&self) -> Vec3<f64> { self.position }
    pub fn getLightColor(&self) -> Rgba { self.color }

    pub fn new(pos: Vec3<f64>, color: Rgba) -> Self {
        Light {
            position: pos,
            color: color
        }
    }

    pub fn default() -> Self {
        Light::new(Vec3::new(0.0, 0.0, 0.0), Rgba::new(1.0, 1.0, 1.0, 0.0))
    }
}
