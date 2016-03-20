extern crate time;
extern crate bmp;
extern crate nalgebra as na;
extern crate palette;

use time::precise_time_ns;
use bmp::{Image, Pixel};
use na::{Vec3, Norm, Cross, Dot};
use palette::{Rgba, named};

mod ray;
mod camera;
mod source;
mod light;
mod sphere;
mod plane;
mod object;

use ray::{Ray};
use camera::{Camera};
use source::{Source};
use light::{Light};
use sphere::{Sphere};
use plane::{Plane};
use object::{Object};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const FWIDTH: f64 = WIDTH as f64;
const FHEIGHT: f64 = HEIGHT as f64;
const ASPECT: f64 = FWIDTH / FHEIGHT;
const AMBIENTLIGHT: f64 = 0.2;
const ACCURACY: f64 = 1e-6;

fn getColorAt(ray: &Ray, scene_obj: &Vec<Box<Object>>, scene_source: &Vec<Box<Source>>, intersect_dis: &Vec<f64>, idx: Option<usize>, acc: f64, amb: f64) -> Rgba {
    let (obj, distance) = match idx {
        None => { return Rgba::new(0.0, 0.0, 0.0, 0.0) },
        Some(idx) => (scene_obj.get(idx).unwrap(), *intersect_dis.get(idx).unwrap())
    };

    let intersect_pos = ray.getRayOrigin() + ray.getRayDirection()*distance;
    let intersect_dir = ray.getRayDirection();

    let mut color = obj.getColor() * (AMBIENTLIGHT as f32);
    let normal = obj.getNormalAt(intersect_pos);

    for light in scene_source.iter() {
        let light_dir = (light.getLightPosition() - intersect_pos).normalize();
        let cos_angle = normal.dot(&light_dir);
        if cos_angle > 0.0 {
            //test for shadows
            let mut shadowed = false;

            let light_dis = (light.getLightPosition() - intersect_pos).norm();
            let shadow_ray = Ray::new(intersect_pos, (light.getLightPosition()-intersect_pos).normalize());

            let mut second_intersect :Vec<f64> = Vec::new();
            for obj in scene_obj.iter() {
                second_intersect.push(obj.findIntersection(&shadow_ray));
            }
            for d in second_intersect {
                if d > ACCURACY && d <= light_dis {
                    // object between light and intersect point
                    shadowed = true;
                }
            }
            if shadowed == false {
                color = color + light.getLightColor() * (cos_angle as f32)
            }
        }
    }
    return color
}

fn winningObjIdx(intersections: &Vec<f64>) -> Option<usize> {
    // return the index of the winning intersection
    match intersections.len() {
        0 => None,
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
                    if (*intersect > 0.0) && (*intersect <= max) {
                        max = *intersect;
                        idx = i;
                    }
                }
                Some(idx)
            } else {
                None
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
    let Z: Vec3<f64> = Vec3::z();

    // declare camera
    let campos = Vec3::new(3.0, 1.5, -4.0);
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
    let mut scene_source: Vec<Box<Source>> = Vec::new();
    scene_source.push(Box::new(Light::new(light_position, white_light)));

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
        let mut intersect_dis: Vec<f64> = Vec::new();

        for obj in scene_obj.iter() {
            intersect_dis.push(obj.findIntersection(&cam_ray));
        }
        let idx = winningObjIdx(&intersect_dis);

        let color = getColorAt(&cam_ray, &scene_obj, &scene_source, &intersect_dis, idx, ACCURACY, AMBIENTLIGHT);

        img.set_pixel(x, y, Pixel{r: (color.red*255.0) as u8, g: (color.green*255.0) as u8, b: (color.blue*255.0) as u8});
    }

    let _ = img.save("scene.bmp");

    let end_time = precise_time_ns();
    println!("Done");

    println!("Running time: {} seconds", (end_time - start_time) as f64 / 10e9);
}
