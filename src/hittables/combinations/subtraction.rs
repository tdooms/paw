use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::hittables::{Container, Hittable};
use crate::util::Bounds3;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subtraction {
    pub first: Box<dyn Hittable>,
    pub second: Box<dyn Hittable>,
}

impl Primitive for Subtraction {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.first.sdf(sample).max(-self.second.sdf(sample))
    }

    fn bounds(&self) -> Bounds3 {
        // self.first.bounds().combine(&self.second.bounds())
        Bounds3::infinite()
    }
}
