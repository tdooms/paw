use crate::materials::Material;
use crate::ray::Hit;
use crate::util::Color3;

#[derive(Debug, Clone)]
pub struct Checkered {
    pub c1: Color3,
    pub c2: Color3,
    pub repeat: f64,
}

impl Material for Checkered {
    fn color(&self, hit: &Hit) -> Color3 {
        let offset = hit.surface.map(|x| (x / self.repeat).floor() as i64).coords;
        // negative numbers?
        match (offset.x + offset.y + offset.z) % 2 {
            1 | -1 => self.c1,
            _ => self.c2,
        }
    }
}
