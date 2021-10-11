use crate::materials::Material;
use crate::ray::Hit;
use crate::util::Color3;
use nalgebra::vector;

#[derive(Debug, Clone)]
pub struct Color {
    pub color: Color3,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            color: vector![0.5, 0.5, 0.5],
        }
    }
}

impl Material for Color {
    fn color(&self, _: &Hit) -> Color3 {
        self.color
    }
}
