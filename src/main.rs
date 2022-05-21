use image::{ImageBuffer, Rgb, RgbImage};

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
}
