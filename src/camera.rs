use crate::{IMAGE_WIDTH, IMAGE_HEIGHT, Vector3, Ray};

pub struct Camera {
    origin: Vector3,
    upper_left_corner: Vector3,
    viewport_width: f64,
    viewport_height: f64,
}

impl Camera {
    pub fn new() -> Camera {
        const ASPECT_RATIO: f64 = (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        const ORIGIN: Vector3 = Vector3::zero();

        Camera {
            origin: ORIGIN,
            upper_left_corner: Vector3 {
                x: -0.5 * VIEWPORT_WIDTH,
                y: 0.5 * VIEWPORT_HEIGHT,
                z: FOCAL_LENGTH,
            } - ORIGIN,
            viewport_width: VIEWPORT_WIDTH,
            viewport_height: VIEWPORT_HEIGHT,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.upper_left_corner + Vector3 {
                x: u * self.viewport_width,
                y: -v * self.viewport_height,
                z: 0.0,
            } - self.origin,
        }
    }
}
