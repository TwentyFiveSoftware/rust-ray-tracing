use crate::{HitRecord, Ray, Vector3};
use crate::scatter_info::ScatterInfo;

#[derive(Copy, Clone)]
pub enum Material {
    NONE,
    DIFFUSE(Vector3),
    METAL(Vector3, f64),
}


impl Material {
    pub fn scatter(self, ray: &Ray, hit_record: &HitRecord) -> ScatterInfo {
        return match self {
            Material::NONE => ScatterInfo::no_scatter(),
            Material::DIFFUSE(albedo) => Material::scatter_diffuse(hit_record, albedo),
            Material::METAL(albedo, fuzz) => Material::scatter_metal(ray, hit_record, albedo, fuzz),
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

    fn reflect(vector: Vector3, normal: Vector3) -> Vector3 {
        vector - normal * 2.0 * vector.dot(normal)
    }
}
