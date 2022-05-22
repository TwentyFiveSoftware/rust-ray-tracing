mod vec3;
mod ray;
mod sphere;
mod hit_record;
mod scene;
mod camera;

use image::{ImageBuffer, Rgb, RgbImage};
use crate::camera::Camera;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::vec3::{Vector3};

const IMAGE_WIDTH: u32 = 500;
const IMAGE_HEIGHT: u32 = 300;

fn main() {
    let camera: Camera = Camera::new();
    let scene: Scene = generate_scene();

    let mut image: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let u: f64 = (x as f64) / (IMAGE_WIDTH as f64);
            let v: f64 = (y as f64) / (IMAGE_HEIGHT as f64);

            let ray: Ray = camera.get_ray(u, v);
            let color: Vector3 = ray_color(&ray, &scene);

            image.put_pixel(x, y, color_to_rgb(color));
        }
    }

    image.save("target/image.png").unwrap();
}

fn color_to_rgb(color: Vector3) -> Rgb<u8> {
    Rgb([(color.x * 256.0) as u8, (color.y * 256.0) as u8, (color.z * 256.0) as u8])
}

fn ray_color(ray: &Ray, scene: &Scene) -> Vector3 {
    let hit_record: HitRecord = ray_hit_scene(scene, &ray, 0.0, f64::INFINITY);
    if hit_record.hit {
        return (hit_record.normal + Vector3 { x: 1.0, y: 1.0, z: 1.0 }) * 0.5;
    }

    let unit_direction: Vector3 = ray.direction.normalized();
    let t: f64 = 0.5 * (unit_direction.y + 1.0);
    let color: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Vector3 { x: 0.5, y: 0.7, z: 1.0 } * t;
    color
}

fn ray_hit_scene(scene: &Scene, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
    let mut current_record: HitRecord = HitRecord::no_hit();
    current_record.t = t_max;

    for sphere in &scene.spheres[..] {
        let record: HitRecord = sphere.ray_hits_sphere(ray, t_min, current_record.t);
        if record.hit {
            current_record = record;
        }
    }

    current_record
}

fn generate_scene() -> Scene {
    let mut spheres: Vec<Sphere> = Vec::new();

    spheres.push(Sphere { center: Vector3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 0.5 });
    spheres.push(Sphere { center: Vector3 { x: 0.0, y: -100.5, z: 1.0 }, radius: 100.0 });

    Scene { spheres }
}
