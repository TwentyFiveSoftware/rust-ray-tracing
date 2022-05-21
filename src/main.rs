mod vec3;

use image::{ImageBuffer, Rgb, RgbImage};
use crate::vec3::Vector3;

fn main() {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 300;

    let mut image: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r: f64 = ((x as f64) / WIDTH as f64) * 256.0;
            let g: f64 = ((y as f64) / WIDTH as f64) * 256.0;
            let b: f64 = 0.25 * 256.0;

            image.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));
        }
    }

    image.save("target/image.png").unwrap();


    let vector1 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    let vector2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };

    println!("{}", vector1);
    println!("{}", vector2);
    println!("{}", -vector1);
    println!("{}", vector1 + vector2);
    println!("{}", vector1 - vector2);
    println!("{}", vector1 * 5.0);
    println!("{}", vector2 / 2.0);
    println!("{}", vector1.length_squared());
    println!("{}", vector1.length());
}
