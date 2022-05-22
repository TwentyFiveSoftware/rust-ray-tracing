use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        Vector3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Self::Output {
        Vector3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f64) -> Self::Output {
        Vector3 { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar }
    }
}

impl Vector3 {
    pub fn dot(self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Vector3 {
        self / self.length()
    }

    pub const fn zero() -> Vector3 {
        return Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    }

    pub fn random_unit_vector() -> Vector3 {
        let mut random = rand::thread_rng();

        loop {
            let vector: Vector3 = Vector3 {
                x: random.gen::<f64>() * 2.0 - 1.0,
                y: random.gen::<f64>() * 2.0 - 1.0,
                z: random.gen::<f64>() * 2.0 - 1.0,
            };

            if vector.length_squared() < 1.0 {
                return vector.normalized();
            }
        }
    }
}
