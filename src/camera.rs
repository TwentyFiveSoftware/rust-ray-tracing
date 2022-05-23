use crate::{IMAGE_WIDTH, IMAGE_HEIGHT, Vector3, Ray};

pub struct Camera {
    origin: Vector3,
    upper_left_corner: Vector3,
    horizontal_direction: Vector3,
    vertical_direction: Vector3,
}

impl Camera {
    pub fn new(look_from: Vector3, look_at: Vector3, fov: f64) -> Camera {
        const ASPECT_RATIO: f64 = (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);

        let viewport_height: f64 = (fov.to_radians() / 2.0).tan() * 2.0;
        let viewport_width: f64 = viewport_height * ASPECT_RATIO;

        let forward: Vector3 = (look_at - look_from).normalized();
        let right: Vector3 = (Vector3 { x: 0.0, y: 1.0, z: 0.0 }).cross(forward).normalized();
        let up: Vector3 = forward.cross(right).normalized();

        let horizontal_direction: Vector3 = right * viewport_width;
        let vertical_direction: Vector3 = up * viewport_height;

        let upper_left_corner: Vector3 = look_from - horizontal_direction * 0.5 + vertical_direction * 0.5 + forward;

        Camera {
            origin: look_from,
            upper_left_corner,
            horizontal_direction,
            vertical_direction,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.upper_left_corner + self.horizontal_direction * u - self.vertical_direction * v - self.origin,
        }
    }
}
