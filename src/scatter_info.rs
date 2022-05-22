use crate::{Ray, Vector3};

pub struct ScatterInfo {
    pub does_scatter: bool,
    pub attenuation: Vector3,
    pub scattered_ray: Ray,
}

impl ScatterInfo {
    pub fn no_scatter() -> ScatterInfo {
        ScatterInfo {
            does_scatter: false,
            attenuation: Vector3::zero(),
            scattered_ray: Ray::none(),
        }
    }
}
