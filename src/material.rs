use rand::Rng;
use crate::{HitRecord, Ray, Vector3};
use crate::scatter_info::ScatterInfo;

#[derive(Copy, Clone)]
pub enum Material {
    NONE,
    DIFFUSE(Vector3),
    METAL(Vector3, f64),
    DIELECTRIC(f64),
}


impl Material {
    pub fn scatter(self, ray: &Ray, hit_record: &HitRecord) -> ScatterInfo {
        return match self {
            Material::NONE => ScatterInfo::no_scatter(),
            Material::DIFFUSE(albedo) => Material::scatter_diffuse(hit_record, albedo),
            Material::METAL(albedo, fuzz) => Material::scatter_metal(ray, hit_record, albedo, fuzz),
            Material::DIELECTRIC(refraction_index) => Material::scatter_dielectric(ray, hit_record, refraction_index)
        };
    }

    fn scatter_diffuse(hit_record: &HitRecord, albedo: Vector3) -> ScatterInfo {
        let mut scatter_direction: Vector3 = hit_record.normal + Vector3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        ScatterInfo {
            does_scatter: true,
            attenuation: albedo,
            scattered_ray: Ray {
                origin: hit_record.point,
                direction: scatter_direction,
            },
        }
    }

    fn scatter_metal(ray: &Ray, hit_record: &HitRecord, albedo: Vector3, fuzz: f64) -> ScatterInfo {
        let scatter_direction: Vector3 = Material::reflect(ray.direction.normalized(), hit_record.normal)
            + Vector3::random_unit_vector() * fuzz;

        ScatterInfo {
            does_scatter: scatter_direction.dot(hit_record.normal) > 0.0,
            attenuation: albedo,
            scattered_ray: Ray {
                origin: hit_record.point,
                direction: scatter_direction,
            },
        }
    }

    fn scatter_dielectric(ray: &Ray, hit_record: &HitRecord, refraction_index: f64) -> ScatterInfo {
        let refraction_ratio: f64 = if hit_record.is_front_face { 1.0 / refraction_index } else { refraction_index };
        let scatter_direction: Vector3 = Material::refract(ray.direction.normalized(), hit_record.normal, refraction_ratio);

        ScatterInfo {
            does_scatter: true,
            attenuation: Vector3 { x: 1.0, y: 1.0, z: 1.0 },
            scattered_ray: Ray {
                origin: hit_record.point,
                direction: scatter_direction,
            },
        }
    }

    fn reflect(vector: Vector3, normal: Vector3) -> Vector3 {
        vector - normal * 2.0 * vector.dot(normal)
    }

    fn refract(vector: Vector3, normal: Vector3, refraction_ratio: f64) -> Vector3 {
        let cos_theta: f64 = normal.dot(-vector).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let r0: f64 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        let reflectance: f64 = r0 * r0 + (1.0 - r0 * r0) * (1.0 - cos_theta).powi(5);

        if refraction_ratio * sin_theta > 1.0 || reflectance > rand::thread_rng().gen::<f64>() {
            return Material::reflect(vector, normal);
        }

        let r_out_perpendicular: Vector3 = (vector + normal * cos_theta) * refraction_ratio;
        let r_out_parallel: Vector3 = normal * -(1.0 - r_out_perpendicular.length_squared()).sqrt();
        r_out_perpendicular + r_out_parallel
    }
}
