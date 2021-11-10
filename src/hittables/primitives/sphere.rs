use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::hittables::Hittable;
use crate::util::Bounds3;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sphere;

impl Hittable for Sphere {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        sample.coords.norm() - 1.0
    }

    fn bounds(&self) -> Bounds3 {
        Bounds3::default()
    }
}
