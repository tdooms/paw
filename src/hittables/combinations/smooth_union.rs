use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::hittables::{Container, Hittable, Object};
use crate::materials::Material;
use crate::util::Bounds3;

#[derive(Debug)]
pub struct SmoothUnion {
    pub first: Box<dyn Object>,
    pub second: Box<dyn Object>,
    pub smoothness: f64,
}

impl Hittable for SmoothUnion {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let d1 = self.first.sdf(sample);
        let d2 = self.second.sdf(sample);

        let h = (self.smoothness - (d1 - d2).abs()).max(0.0);
        d1.min(d2) - h * h * 0.25 / self.smoothness
    }

    fn bounds(&self) -> Bounds3 {
        // self.first.bounds().combine(&self.second.bounds())
        Bounds3::infinite()
    }
}
