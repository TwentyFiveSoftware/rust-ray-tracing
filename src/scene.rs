use crate::{HitRecord, Material, Ray, Sphere, Vector3};
use crate::color_util::random_color;
use crate::texture::Texture;

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn generate() -> Scene {
        let mut spheres: Vec<Sphere> = Vec::new();

        // GROUND
        spheres.push(Sphere {
            center: Vector3 { x: 0.0, y: -1000.0, z: 1.0 },
            radius: 1000.0,
            material: Material::DIFFUSE(Texture::CHECKERED(
                Vector3 { x: 0.05, y: 0.05, z: 0.05 }, Vector3 { x: 0.95, y: 0.95, z: 0.95 })),
        });

        let random = fastrand::Rng::new();

        // SMALL SPHERES
        for x in -11..11 {
            for z in -11..11 {
                let material_random: f64 = random.f64();

                let material: Material =
                    if material_random < 0.8 {
                        Material::DIFFUSE(Texture::SOLID(random_color()))
                    } else if material_random < 0.95 {
                        Material::METAL(Texture::SOLID(random_color()), 0.0)
                    } else {
                        Material::DIELECTRIC(1.5)
                    };

                spheres.push(Sphere {
                    center: Vector3 {
                        x: x as f64 + 0.9 * random.f64(),
                        y: 0.2,
                        z: z as f64 + 0.9 * random.f64(),
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
            material: Material::DIFFUSE(Texture::SOLID(Vector3 { x: 0.6, y: 0.3, z: 0.1 })),
        });

        spheres.push(Sphere {
            center: Vector3 { x: 4.0, y: 1.0, z: 0.0 },
            radius: 1.0,
            material: Material::METAL(Texture::SOLID(Vector3 { x: 0.7, y: 0.6, z: 0.5 }), 0.0),
        });

        Scene { spheres }
    }

    pub fn ray_hit_scene(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let mut current_record: HitRecord = HitRecord::no_hit();
        current_record.t = t_max;

        for sphere in self.spheres.iter() {
            let record: HitRecord = sphere.ray_hits_sphere(ray, t_min, current_record.t);
            if record.hit {
                current_record = record;
            }
        }

        current_record
    }
}
