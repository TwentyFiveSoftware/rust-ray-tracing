mod vec3;
mod ray;
mod sphere;
mod hit_record;

use image::{ImageBuffer, Rgb, RgbImage};
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Vector3};

fn main() {
    const WIDTH: u32 = 500;
    const HEIGHT: u32 = 300;

    const ASPECT_RATIO: f64 = (WIDTH as f64) / (HEIGHT as f64);
    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;

    const ORIGIN: Vector3 = Vector3::zero();
    const UPPER_LEFT_CORNER_OFFSET: Vector3 =
        Vector3 { x: -0.5 * VIEWPORT_WIDTH, y: 0.5 * VIEWPORT_HEIGHT, z: FOCAL_LENGTH };

    let mut image: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let u: f64 = (x as f64) / (WIDTH as f64);
            let v: f64 = (y as f64) / (HEIGHT as f64);

            let direction: Vector3 = UPPER_LEFT_CORNER_OFFSET
                + Vector3 { x: u * VIEWPORT_WIDTH, y: -v * VIEWPORT_HEIGHT, z: 0.0 };
            let ray: Ray = Ray { origin: ORIGIN, direction };

            let color: Vector3 = ray_color(ray);

            image.put_pixel(x, y, color_to_rgb(color));
        }
    }

    image.save("target/image.png").unwrap();
}

fn color_to_rgb(color: Vector3) -> Rgb<u8> {
    Rgb([(color.x * 256.0) as u8, (color.y * 256.0) as u8, (color.z * 256.0) as u8])
}

fn ray_color(ray: Ray) -> Vector3 {
    let sphere: Sphere = Sphere { center: Vector3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 0.5 };

    let hit_record: HitRecord = sphere.ray_hits_sphere(&ray, 0.0, f64::INFINITY);
    if hit_record.hit {
        return (hit_record.normal + Vector3{x: 1.0, y: 1.0, z: 1.0}) * 0.5;
    }

    let unit_direction: Vector3 = ray.direction.normalized();
    let t: f64 = 0.5 * (unit_direction.y + 1.0);
    let color: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Vector3 { x: 0.5, y: 0.7, z: 1.0 } * t;
    color
}
