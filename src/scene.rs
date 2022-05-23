use rand::Rng;
use crate::{Material, Sphere, Vector3};

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn generate() -> Scene {
        let mut spheres: Vec<Sphere> = Vec::new();

        // GROUND
        spheres.push(Sphere {
            center: Vector3 { x: 0.0, y: -1000.5, z: 1.0 },
            radius: 1000.0,
            material: Material::DIFFUSE(Vector3 { x: 0.05, y: 0.05, z: 0.05 }),
        });

        let mut random = rand::thread_rng();

        // SMALL SPHERES
        for x in -11..11 {
            for z in -11..11 {
                let material_random: f64 = random.gen::<f64>();

                let material: Material =
                    if material_random < 0.8 {
                        Material::DIFFUSE(random_color())
                    } else if material_random < 0.95 {
                        Material::METAL(random_color(), 0.0)
                    } else {
                        Material::DIELECTRIC(1.5)
                    };

                spheres.push(Sphere {
                    center: Vector3 {
                        x: x as f64 + 0.9 * random.gen::<f64>(),
                        y: 0.2,
                        z: z as f64 + 0.9 * random.gen::<f64>(),
                    },
                    radius: 0.2,
                    material,
                });
            }
        }

        // BIG SPHERES
        spheres.push(Sphere {
            center: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
            radius: 1.0,
            material: Material::DIELECTRIC(1.5),
        });

        spheres.push(Sphere {
            center: Vector3 { x: -4.0, y: 1.0, z: 0.0 },
            radius: 1.0,
            material: Material::DIFFUSE(Vector3 { x: 0.6, y: 0.3, z: 0.1 }),
        });

        spheres.push(Sphere {
            center: Vector3 { x: 4.0, y: 1.0, z: 0.0 },
            radius: 1.0,
            material: Material::METAL(Vector3 { x: 0.7, y: 0.6, z: 0.5 }, 0.0),
        });

        Scene { spheres }
    }
}

fn random_color() -> Vector3 {
    let h: f64 = (rand::thread_rng().gen::<f64>() * 360.0).floor();
    hsv_to_rgb(h, 0.75, 0.45)
}

pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Vector3 {
    let c: f64 = s * v;
    let x: f64 = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m: f64 = v - c;

    let r: f64;
    let g: f64;
    let b: f64;

    if h >= 0.0 && h < 60.0 {
        r = c;
        g = x;
        b = 0.0;
    } else if h >= 60.0 && h < 120.0 {
        r = x;
        g = c;
        b = 0.0;
    } else if h >= 120.0 && h < 180.0 {
        r = 0.0;
        g = c;
        b = x;
    } else if h >= 180.0 && h < 240.0 {
        r = 0.0;
        g = x;
        b = c;
    } else if h >= 240.0 && h < 300.0 {
        r = x;
        g = 0.0;
        b = c;
    } else {
        r = c;
        g = 0.0;
        b = x;
    }

    Vector3 { x: r + m, y: g + m, z: b + m }
}
