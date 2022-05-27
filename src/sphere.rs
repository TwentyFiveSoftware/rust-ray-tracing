use crate::{Ray, Vector3};
use crate::hit_record::HitRecord;
use crate::material::Material;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn ray_hits_sphere(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let oc: Vector3 = &ray.origin - &self.center;
        let a: f64 = ray.direction.length_squared();
        let half_b: f64 = oc.dot(&ray.direction);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return HitRecord::no_hit();
        }

        let sqrt_d: f64 = discriminant.sqrt();
        let mut t: f64 = (-half_b - sqrt_d) / a;
        if t < t_min || t > t_max {
            t = (-half_b + sqrt_d) / a;

            if t < t_min || t > t_max {
                return HitRecord::no_hit();
            }
        }

        let point: Vector3 = ray.at(t);
        let normal: Vector3 = (&point - &self.center) / self.radius;
        let is_front_face: bool = ray.direction.dot(&normal) < 0.0;

        return HitRecord {
            hit: true,
            t,
            point,
            normal: if is_front_face { normal } else { -&normal },
            is_front_face,
            material: self.material.clone(),
        };
    }
}
