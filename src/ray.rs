use nalgebra::{Point3, UnitVector3, Unit};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3<f64>,
    direction: UnitVector3<f64>,
}

impl Ray {
    pub fn to(from: Point3<f64>, to: Point3<f64>) -> Self {
        Self { origin: from, direction: Unit::new_normalize(to - from)}
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction.scale(t)
    }
}