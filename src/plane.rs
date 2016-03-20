extern crate nalgebra as na;
extern crate palette;

use na::{Vec3, Dot};
use palette::{Rgb, Rgba};
use ray::{Ray};

pub struct Plane {
    normal: Vec3<f64>,
    distance: f64,
    color: Rgba,
}

impl Plane {
    pub fn getPlaneNormal(&self) -> Vec3<f64> { self.normal }
    pub fn getPlaneDistance(&self) -> f64 { self.distance }
    pub fn getPlaneColor(&self) -> Rgba { self.color }

    pub fn getNormalAt(&self, point: Vec3<f64>) -> Vec3<f64> { self.normal }
    pub fn findIntersection(&self, ray: Ray) -> f64 {
        let ray_direction = ray.getRayDirection();
        let a: f64 = ray_direction.dot(&self.normal);
        // a == 0 then ray is parallel to the plane
        if a < 1e-6  {
            -1.0
        } else {
            -1.0 * (self.normal.dot(&(ray.getRayOrigin() - (self.normal * self.distance)))) / a
        }
    }

    pub fn new(norm: Vec3<f64>, d: f64, color: Rgba) -> Self {
        Plane {
            normal: norm,
            distance: d,
            color: color,
        }
    }

    pub fn default() -> Self {
        Plane::new(
            Vec3::new(1.0, 0.0, 0.0), 0.0, Rgba::new(0.5, 0.5, 0.5, 0.0)
        )
    }
}
