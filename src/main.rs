extern crate time;
extern crate bmp;
extern crate nalgebra as na;
extern crate palette;

use time::precise_time_ns;
use bmp::{Image, Pixel};
use na::{Vec3, Norm, Cross};
use palette::{Rgba};

mod ray;
mod camera;
mod light;
mod sphere;
mod plane;
mod object;

use ray::{Ray};
use camera::{Camera};
use light::{Light};
use sphere::{Sphere};
use plane::{Plane};
use object::{Object};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const FWIDTH: f64 = WIDTH as f64;
const FHEIGHT: f64 = HEIGHT as f64;
const ASPECT: f64 = FWIDTH / FHEIGHT;

fn winningObjIdx(intersections: Vec<f64>) -> i32 {
    // return the index of the winning intersection
    match intersections.len() {
        0 => -1,
        _ => {
            let mut max = 0.0f64;
            let mut idx:usize = 0;
            for intersect in intersections.iter() {
                if *intersect > max {
                    max = *intersect
                }
            }
            if max > 0.0 {
                for (i, intersect) in intersections.iter().enumerate() {
                    if *intersect > 0.0 && *intersect < max {
                        max = *intersect;
                        idx = i;
                    }
                }
                idx as i32
            } else {
                -1
            }
        },
    }
}

fn main() {
    println!("rendering ...");
    let start_time = precise_time_ns();

    let mut img = Image::new(WIDTH, HEIGHT);

    // declare X, Y, Z unit vector
    let O = Vec3::new(0.0, 0.0, 0.0);
    let X: Vec3<f64> = Vec3::x();
    let Y: Vec3<f64> = Vec3::y();
    let Y: Vec3<f64> = Vec3::z();

    // declare camera
    let campos = Vec3::new(2.0, 1.5, -4.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let diff_btw = campos - look_at;
    let camdir = -diff_btw.normalize();
    let camright = Y.cross(&camdir).normalize();
    let camdown = camright.cross(&camdir);

    let Camera = Camera::new(campos, camdir, camright, camdown);

    // declare light
    let white_light = Rgba::new(1.0, 1.0, 1.0, 0.0);
    let pretty_green = Rgba::new(0.5, 1.0, 0.5, 0.3);
    let maroon = Rgba::new(0.5, 0.25, 0.25, 0.0);
    let gray = Rgba::new(0.5, 0.5, 0.5, 0.0);
    let black = Rgba::new(0.0, 0.0, 0.0, 0.0);

    let light_position = Vec3::new(-7.0, 10.0, -10.0);
    let scene_light = Light::new(light_position, white_light);

    // declare Scene
    let mut scene_obj: Vec<Box<Object>> = Vec::new();
    scene_obj.push(Box::new(Sphere::new(O, 1.0, pretty_green)));
    scene_obj.push(Box::new(Plane::new(Y, -1.0, maroon)));

    let mut xamnt: f64 = 0.0;
    let mut yamnt: f64 = 0.0;
    let cam_ray_origin = Camera.getCameraPosition();

    for (x, y) in img.coordinates() {
        // start with no-AA
        let fx = x as f64;
        let fy = y as f64;
        if ASPECT > 1.0 {
            xamnt = ((fx+0.5)/FWIDTH)*ASPECT - ((FWIDTH-FHEIGHT) / FHEIGHT / 2f64);
            yamnt = ((FHEIGHT - fy) + 0.5)/FHEIGHT;
        } else if ASPECT < 1.0 {
            xamnt = (fx+0.5)/FWIDTH;
            yamnt = (((FHEIGHT - fy)+0.5)/ FHEIGHT)/ASPECT - ((FHEIGHT-FWIDTH)/ FWIDTH/2.0);
        } else {
            xamnt = (fx+0.5)/FWIDTH;
            yamnt = ((FHEIGHT - fy) + 0.5)/FHEIGHT;
        }

        let cam_ray_direction = (camdir + (camright*(xamnt - 0.5)) + camdown*(yamnt-0.5)).normalize();
        let cam_ray = Ray::new(cam_ray_origin, cam_ray_direction);
        let mut intersections: Vec<f64> = Vec::new();

        for obj in scene_obj.iter() {
            intersections.push(obj.findIntersection(&cam_ray));
        }
        let idx = winningObjIdx(intersections);
    }

    let _ = img.save("scene.bmp");

    let end_time = precise_time_ns();
    println!("Done");

    println!("Running time: {} seconds", (end_time - start_time) as f64 / 10e9);
}
