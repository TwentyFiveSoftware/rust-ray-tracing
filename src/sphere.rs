use crate::{Ray, Vector3};

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

impl Sphere {
    pub fn ray_hits_sphere(&self, ray: &Ray) -> bool {
        let oc: Vector3 = ray.origin - self.center;
        let a: f64 = ray.direction.length_squared();
        let half_b: f64 = oc.dot(ray.direction);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;

        return discriminant > 0.0;
    }
}
