use crate::lights::base::Light;
use nalgebra::Point3;

pub struct PointLight {
    pub position: Point3<f64>,
    pub color: Point3<f64>,
    pub softness: f64,
}

impl Light for PointLight {
    fn sample(&self, point: Point3<f64>) -> f64 {
        1.0
    }
}
