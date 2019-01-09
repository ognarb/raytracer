extern crate nalgebra as na;

// imageBuffer
extern crate image;

// parse wavefront_obj
extern crate wavefront_obj;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use crate::world::sphere::Sphere;
use crate::world::light::Light;
use image::{Pixel, Rgba};
use na::Vector3;
use crate::world::InterceptableEnum;

mod helpers;
mod intersection;
mod ray;
mod world;

fn main() {
    /*let img = ImageBuffer::new(512, 512);
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
        }
    }*/

    let mut elements = Vec::new();
    elements.push(InterceptableEnum::Sphere(Sphere {
        center: Vector3::new(0.0, 0.0, 5.0),
        radius: 1.0,
        color: Rgba::from_channels(0.3, 0.3, 0.3, 1.0),
        opacity: 1.0,
        reflection: 0.0,
    }));
    let mut lights = Vec::new();
    lights.push(Light::new(10.0, 10.0, 10.0));

    let w = world::World::new(300, 200, elements, lights);
    let string = serde_json::to_string(&w).unwrap();
    //w.render().save(io::stdout(), image::ImageFormat::PNG);
    let image = w.render();
    image.save("./output.png");
    println!("{}", string); 
}
