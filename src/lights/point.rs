use crate::lights::base::Light;
use crate::util::Color3;
use nalgebra::Point3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PointLight {
    pub position: Point3<f64>,
    pub color: Color3,
    pub softness: f64,
}

impl Light for PointLight {
    fn sample(&self, point: Point3<f64>) -> f64 {
        1.0
    }
}
