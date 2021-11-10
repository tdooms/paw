use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::hittables::Hittable;
use crate::util::Bounds3;

#[derive(Debug, Serialize, Deserialize)]
pub struct Plane;

impl Hittable for Plane {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        sample.y
    }

    fn bounds(&self) -> Bounds3 {
        // TODO: eh...
        Bounds3::infinite()
    }
}
