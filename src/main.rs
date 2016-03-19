extern crate time;

use time::precise_time_ns;

fn main() {
    println!("rendering ...");
    let start_time = precise_time_ns();
    let end_time = precise_time_ns();
    println!("Done");

    println!("Running time: {} seconds", (end_time - start_time) as f64 / 10e9);
}
