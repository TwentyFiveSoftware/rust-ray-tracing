use crate::{Material, Vector3};

pub struct HitRecord {
    pub hit: bool,
    pub t: f64,
    pub point: Vector3,
    pub normal: Vector3,
    pub is_front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn no_hit() -> HitRecord {
        HitRecord {
            hit: false,
            t: 0.0,
            point: Vector3::zero(),
            normal: Vector3::zero(),
            is_front_face: true,
            material: Material::NONE,
        }
    }
}
