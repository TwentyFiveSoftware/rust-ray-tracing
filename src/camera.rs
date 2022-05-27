use crate::{IMAGE_WIDTH, IMAGE_HEIGHT, Vector3, Ray};

pub struct Camera {
    look_from: Vector3,
    upper_left_corner: Vector3,
    horizontal_direction: Vector3,
    vertical_direction: Vector3,
    up_vector: Vector3,
    right_vector: Vector3,
    aperture: f64,
}

impl Camera {
    pub fn new(look_from: Vector3, look_at: Vector3, fov: f64, aperture: f64, focus_distance: f64) -> Camera {
        const ASPECT_RATIO: f64 = (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);

        let viewport_height: f64 = (fov.to_radians() / 2.0).tan() * 2.0;
        let viewport_width: f64 = viewport_height * ASPECT_RATIO;

        let forward: Vector3 = (&look_at - &look_from).normalized();
        let right: Vector3 = Vector3::up().cross(&forward).normalized();
        let up: Vector3 = forward.cross(&right).normalized();

        let horizontal_direction: Vector3 = &right * viewport_width * focus_distance;
        let vertical_direction: Vector3 = &up * viewport_height * focus_distance;

        let upper_left_corner: Vector3 = &look_from - &(&horizontal_direction * 0.5 + &vertical_direction * 0.5 + forward * focus_distance);

        Camera {
            look_from,
            upper_left_corner,
            horizontal_direction,
            vertical_direction,
            up_vector: up,
            right_vector: right,
            aperture,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let random: Vector3 = Vector3::random_unit_vector() * (self.aperture * 0.5);
        let offset: Vector3 = &self.up_vector * random.y + &self.right_vector * random.x;

        let origin: Vector3 = &self.look_from + &offset;
        let target: Vector3 = &self.upper_left_corner + &(&self.horizontal_direction * u - &self.vertical_direction * v);

        Ray {
            direction: &target - &origin,
            origin,
        }
    }
}
