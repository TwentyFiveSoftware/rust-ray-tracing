use crate::Vector3;

pub struct HitRecord {
    pub hit: bool,
    pub normal: Vector3,
    pub t: f64,
    pub point: Vector3,
}

impl HitRecord {
    pub fn no_hit() -> HitRecord {
        HitRecord {
            hit: false,
            normal: Vector3::zero(),
            t: 0.0,
            point: Vector3::zero(),
        }
    }
}
