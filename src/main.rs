mod vec3;
mod ray;
mod sphere;
mod hit_record;
mod scene;
mod camera;
mod material;
mod scatter_info;
mod texture;
mod renderer;
mod color_util;

use std::sync::{Arc, mpsc, Mutex};
use std::{thread};
use std::time::{Duration, Instant};
use image::{ImageBuffer, RgbImage};
use clap::Parser;
use crate::camera::Camera;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::renderer::render_row;
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

                let row = render_row(y, args.samples_per_pixel, &thread_camera, &thread_scene);
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
