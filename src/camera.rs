extern crate nalgebra as na;

use na::{Vec3};

pub struct Camera {
    campos: Vec3<f64>,
    camdir: Vec3<f64>,
    camright: Vec3<f64>,
    camdown: Vec3<f64>,
}

impl Camera {
    pub fn getCameraPosition(&self) -> Vec3<f64> { self.campos }
    pub fn getCameraDirection(&self) -> Vec3<f64> { self.camdir }
    pub fn getCameraRight(&self) -> Vec3<f64> { self.camright }
    pub fn getCameraDown(&self) -> Vec3<f64> { self.camdown }

    pub fn new(pos: Vec3<f64>, dir: Vec3<f64>, right: Vec3<f64>, down: Vec3<f64>) -> Self {
        Camera {
            campos: pos,
            camdir: dir,
            camright: right,
            camdown: down,
        }
    }

    pub fn default(&self) -> Self {
        Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        )
    }
}
