extern crate nalgebra as na;
extern crate palette;

use na::{Vec3};
use palette::{Rgb, Rgba};

pub trait Source {
    fn getLightPosition(&self) -> Vec3<f64>;
    fn getLightColor(&self) -> Rgba; 
}
