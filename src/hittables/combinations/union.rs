use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::hittables::Hittable;
use crate::object::Object;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};

#[derive(Debug, Serialize, Deserialize)]
pub struct Union {
    pub first: Object,
    pub second: Object,
}

#[typetag::serde(name = "union")]
impl Hittable for Union {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.first.sdf(sample).min(self.second.sdf(sample))
    }

    fn bounds(&self) -> Bounds3 {
        // self.first.bounds().combine(&self.second.bounds())
        Bounds3::infinite()
    }
}
