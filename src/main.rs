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

use ray::{Ray};
use camera::{Camera};
use light::{Light};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    println!("rendering ...");
    let start_time = precise_time_ns();

    let mut img = Image::new(WIDTH, HEIGHT);

    // declare X, Y, Z unit vector
    let X = Vec3::new(1.0, 0.0, 0.0);
    let Y = Vec3::new(0.0, 1.0, 0.0);
    let Y = Vec3::new(0.0, 0.0, 1.0);

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
    let gray = Rgba::new(0.5, 0.5, 0.5, 0.0);
    let black = Rgba::new(0.0, 0.0, 0.0, 0.0);

    let light_position = Vec3::new(-7.0, 10.0, -10.0);
    let scene_light = Light::new(light_position, white_light);

    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, Pixel{r: 127, g: 127, b: 127});
    }

    let _ = img.save("scene.bmp");

    let end_time = precise_time_ns();
    println!("Done");

    println!("Running time: {} seconds", (end_time - start_time) as f64 / 10e9);
}
