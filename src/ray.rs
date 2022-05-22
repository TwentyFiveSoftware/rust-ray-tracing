use crate::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn none() -> Ray {
        Ray {
            origin: Vector3::zero(),
            direction: Vector3::zero(),
        }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }
}
