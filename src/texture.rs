use crate::Vector3;

#[derive(Clone)]
pub enum Texture {
    SOLID(Vector3),
    CHECKERED(Vector3, Vector3),
}

impl Texture {
    pub fn get_color(&self, point: &Vector3) -> Vector3 {
        return match self {
            Texture::SOLID(albedo) => albedo.clone(),
            Texture::CHECKERED(odd, even) => {
                let size: f64 = 6.0;
                let sin: f64 = (size * point.x).sin() * (size * point.y).sin() * (size * point.z).sin();
                return (if sin < 0.0 { odd } else { even }).clone();
            }
        };
    }
}
