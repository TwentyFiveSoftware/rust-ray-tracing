use image::Rgb;
use crate::{Camera, HitRecord, IMAGE_HEIGHT, IMAGE_WIDTH, MAX_RAY_TRACE_DEPTH, Ray, ScatterInfo, Scene, Vector3};

pub fn render_row(y: u32, samples_per_pixel: u32, camera: &Camera, scene: &Scene) -> [Rgb<u8>; IMAGE_WIDTH as usize] {
    let mut row: [Rgb<u8>; IMAGE_WIDTH as usize] = [Rgb([0, 0, 0]); IMAGE_WIDTH as usize];

    for x in 0..IMAGE_WIDTH {
        let mut pixel_color: Vector3 = Vector3::zero();

        for _ in 0..samples_per_pixel {
            let u: f64 = (x as f64 + fastrand::f64()) / (IMAGE_WIDTH as f64 - 1.0);
            let v: f64 = (y as f64 + fastrand::f64()) / (IMAGE_HEIGHT as f64 - 1.0);

            let ray: Ray = camera.get_ray(u, v);
            pixel_color = pixel_color + ray_color(&scene, &ray, MAX_RAY_TRACE_DEPTH);
        }

        pixel_color = pixel_color / (samples_per_pixel as f64);
        row[x as usize] = color_to_rgb(pixel_color);
    }

    row
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

fn color_to_rgb(mut color: Vector3) -> Rgb<u8> {
    color.x = color.x.sqrt();
    color.y = color.y.sqrt();
    color.z = color.z.sqrt();

    color.x = if color.x > 1.0 { 1.0 } else if color.x < 0.0 { 0.0 } else { color.x };
    color.y = if color.y > 1.0 { 1.0 } else if color.y < 0.0 { 0.0 } else { color.y };
    color.z = if color.z > 1.0 { 1.0 } else if color.z < 0.0 { 0.0 } else { color.z };

    color = color * 256.0;

    Rgb([color.x as u8, color.y as u8, color.z as u8])
}
