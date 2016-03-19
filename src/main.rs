extern crate time;
extern crate bmp;

use time::precise_time_ns;
use bmp::{Image, Pixel};

mod ray;
mod camera;

use ray::{Ray};
use camera::{Camera};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    println!("rendering ...");
    let start_time = precise_time_ns();

    let mut img = Image::new(WIDTH, HEIGHT);

    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, Pixel{r: 127, g: 127, b: 127});
    }

    let _ = img.save("scene.bmp");

    let end_time = precise_time_ns();
    println!("Done");

    println!("Running time: {} seconds", (end_time - start_time) as f64 / 10e9);
}
