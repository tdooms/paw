use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::hittables::{Container, Hittable, Object};
use crate::materials::Material;
use crate::util::Bounds3;

#[derive(Debug)]
pub struct Intersection {
    pub first: Box<dyn Object>,
    pub second: Box<dyn Object>,
}

impl Hittable for Intersection {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.first.sdf(sample).max(self.second.sdf(sample))
    }

    fn bounds(&self) -> Bounds3 {
        // self.first.bounds().combine(&self.second.bounds())
        Bounds3::infinite()
    }
}
