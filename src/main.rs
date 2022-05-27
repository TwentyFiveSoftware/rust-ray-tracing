mod vec3;
mod ray;
mod sphere;
mod hit_record;
mod scene;
mod camera;
mod material;
mod scatter_info;
mod texture;

use std::sync::{Arc, mpsc, Mutex};
use std::{thread};
use std::time::{Duration, Instant};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::Rng;
use clap::Parser;
use crate::camera::Camera;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::scatter_info::ScatterInfo;
use crate::scene::{Scene};
use crate::sphere::Sphere;
use crate::vec3::{Vector3};

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;
const MAX_RAY_TRACE_DEPTH: u32 = 50;
const DEFAULT_SAMPLES_PER_PIXEL: u32 = 100;

#[derive(Parser, Debug)]
struct Args {
    /// Amount of samples per pixel to calculate
    #[clap(short, long, default_value_t = DEFAULT_SAMPLES_PER_PIXEL)]
    samples_per_pixel: u32,

    /// Number of render threads
    #[clap(short, long, default_value_t = num_cpus::get() as u32)]
    threads: u32,
}

fn main() {
    let mut args: Args = Args::parse();
    args.threads = args.threads.max(1);

    let camera = Arc::new(Camera::new(
        Vector3 { x: 12.0, y: 2.0, z: -3.0 },
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        25.0,
        0.0,
        10.0,
    ));

    let scene = Arc::new(Scene::generate());

    let mut image: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let next_row = Arc::new(Mutex::new(0));
    let (rows_sender, rows_receiver) = mpsc::channel();
    let mut handles = Vec::new();

    let render_start_time = Instant::now();

    for _ in 0..args.threads {
        let thread_rows_sender = rows_sender.clone();
        let thread_next_row = Arc::clone(&next_row);
        let thread_scene = Arc::clone(&scene);
        let thread_camera = Arc::clone(&camera);

        let handle = thread::spawn(move || {
            loop {
                let mut next_row = thread_next_row.lock().unwrap();
                if *next_row >= IMAGE_HEIGHT {
                    break;
                }

                let y = *next_row;
                *next_row += 1;
                drop(next_row);

                println!("{} / {} ({:.2}%)", y + 1, IMAGE_HEIGHT, ((y + 1) as f64) * 100.0 / IMAGE_HEIGHT as f64);

                let mut row: [Rgb<u8>; IMAGE_WIDTH as usize] = [Rgb([0, 0, 0]); IMAGE_WIDTH as usize];
                let mut random = rand::thread_rng();

                for x in 0..IMAGE_WIDTH {
                    let mut pixel_color: Vector3 = Vector3::zero();

                    for _ in 0..args.samples_per_pixel {
                        let u: f64 = (x as f64 + random.gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                        let v: f64 = (y as f64 + random.gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);

                        let ray: Ray = thread_camera.get_ray(u, v);
                        pixel_color = pixel_color + ray_color(&thread_scene, &ray, MAX_RAY_TRACE_DEPTH);
                    }

                    row[x as usize] = color_to_rgb(pixel_color, args.samples_per_pixel);
                }

                thread_rows_sender.send((y, row)).unwrap();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_render_time: Duration = render_start_time.elapsed();
    println!("Rendered {} samples/pixel with {} threads in {} ms",
             args.samples_per_pixel, args.threads, elapsed_render_time.as_millis());

    let mut rows_processed: u32 = 0;

    for (y, row) in rows_receiver {
        for x in 0..row.len() {
            image.put_pixel(x as u32, y, row[x]);
        }

        rows_processed += 1;
        if rows_processed >= IMAGE_HEIGHT {
            break;
        }
    }

    image.save("render.png").unwrap();
}

fn color_to_rgb(mut color: Vector3, samples: u32) -> Rgb<u8> {
    color = color / (samples as f64);

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
            return &scatter_info.attenuation * ray_color(scene, &scatter_info.scattered_ray, depth - 1);
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
