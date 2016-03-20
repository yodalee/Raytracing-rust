extern crate nalgebra as na;
extern crate palette;

use na::{Vec3, Norm, Dot};
use palette::{Rgb, Rgba};
use std::num;

use object::{Object};
use ray::{Ray};

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

impl Object for Sphere {
    fn getNormalAt(&self, point: Vec3<f64>) -> Vec3<f64> {
        // Normal point from center to point
        (point - self.getSphereCenter()).normalize()
    }

    fn findIntersection(&self, ray: &Ray) -> f64 {
        let b = 2.0*(ray.getRayOrigin() - self.getSphereCenter()).dot(&ray.getRayDirection());
        let c = (ray.getRayOrigin() - self.getSphereCenter()).sqnorm() - self.getSphereRadius()*self.getSphereRadius();

        let discriminant = b*b - 4.0 * c;
        if discriminant > 0.0 {
            // the ray intersect the sphere at two point
            let root1 = ((-1.0*b - discriminant.sqrt())/2.0) - 0.001;
            if root1 > 0.0 {
                // the first root is the smallest positive root
                root1
            } else {
                ((-1.0*b + discriminant.sqrt())/2.0)-0.001
            }
        } else {
            // no intersect
            -1.0
        }
    }
}
