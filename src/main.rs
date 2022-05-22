mod vec3;
mod ray;
mod sphere;
mod hit_record;
mod scene;
mod camera;
mod material;
mod scatter_info;

use image::{ImageBuffer, Rgb, RgbImage};
use rand::Rng;
use crate::camera::Camera;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::scatter_info::ScatterInfo;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::vec3::{Vector3};

const IMAGE_WIDTH: u32 = 500;
const IMAGE_HEIGHT: u32 = 300;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_RAY_TRACE_DEPTH: u32 = 50;

fn main() {
    let camera: Camera = Camera::new();
    let scene: Scene = generate_scene();

    let mut image: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut random = rand::thread_rng();

    for y in 0..IMAGE_HEIGHT {
        println!("{} / {} ({:.2}%)", y + 1, IMAGE_HEIGHT, ((y + 1) as f64) * 100.0 / IMAGE_HEIGHT as f64);

        for x in 0..IMAGE_WIDTH {
            let mut pixel_color: Vector3 = Vector3::zero();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (x as f64 + random.gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                let v: f64 = (y as f64 + random.gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);

                let ray: Ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&scene, &ray, MAX_RAY_TRACE_DEPTH);
            }

            image.put_pixel(x, y, color_to_rgb(pixel_color));
        }
    }

    image.save("target/image.png").unwrap();
}

fn color_to_rgb(mut color: Vector3) -> Rgb<u8> {
    color = color / SAMPLES_PER_PIXEL as f64;

    color.x = color.x.sqrt();
    color.y = color.y.sqrt();
    color.z = color.z.sqrt();

    color.x = if color.x > 1.0 { 1.0 } else if color.x < 0.0 { 0.0 } else { color.x };
    color.y = if color.y > 1.0 { 1.0 } else if color.y < 0.0 { 0.0 } else { color.y };
    color.z = if color.z > 1.0 { 1.0 } else if color.z < 0.0 { 0.0 } else { color.z };

    color = color * 256.0;

    Rgb([color.x as u8, color.y as u8, color.z as u8])
}

fn ray_color(scene: &Scene, ray: &Ray, depth: u32) -> Vector3 {
    if depth <= 0 {
        return Vector3::zero();
    }

    let hit_record: HitRecord = ray_hit_scene(scene, &ray, 0.001, f64::INFINITY);
    if hit_record.hit {
        let scatter_info: ScatterInfo = hit_record.material.scatter(ray, &hit_record);

        if scatter_info.does_scatter {
            return scatter_info.attenuation * ray_color(scene, &scatter_info.scattered_ray, depth - 1);
        }

        return Vector3::zero();
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

    // ground
    spheres.push(Sphere {
        center: Vector3 { x: 0.0, y: -100.5, z: 1.0 },
        radius: 100.0,
        material: Material::DIFFUSE(Vector3 { x: 0.8, y: 0.8, z: 0.0 }),
    });

    // center
    spheres.push(Sphere {
        center: Vector3 { x: 0.0, y: 0.0, z: 1.0 },
        radius: 0.5,
        material: Material::DIFFUSE(Vector3 { x: 0.1, y: 0.2, z: 0.5 }),
    });

    // left
    spheres.push(Sphere {
        center: Vector3 { x: -1.0, y: 0.0, z: 1.0 },
        radius: 0.5,
        material: Material::DIELECTRIC(1.5),
    });

    // right
    spheres.push(Sphere {
        center: Vector3 { x: 1.0, y: 0.0, z: 1.0 },
        radius: 0.5,
        material: Material::METAL(Vector3 { x: 0.8, y: 0.6, z: 0.2 }, 0.0),
    });

    Scene { spheres }
}
