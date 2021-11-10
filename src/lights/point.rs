use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::lights::base::Light;
use crate::util::Color3;

#[derive(Serialize, Deserialize)]
pub struct PointLight {
    pub position: Point3<f64>,
    pub color: Color3,
    pub softness: f64,
}

impl Light for PointLight {
    fn sample(&self, _point: Point3<f64>) -> f64 {
        1.0
    }
}
