#![allow(dead_code)]

mod tuple;
mod color;
mod utils;
mod canvas;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut canvas = crate::canvas::Canvas::new(1000, 1000);

    let red = crate::color::Color::new(1., 0., 0.);

    for i in 0..1000 {
        canvas.set_pixel(i, i, red);
    }

    let path = Path::new("out.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(canvas.to_ppm().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    let _ = Command::new("open").arg("out.ppm").output();


}
