use nalgebra::Point3;

use crate::hittables::base::Hittable;
use crate::hittables::{Attributes, Container};
use crate::materials::Material;
use crate::util::{Bounds3, Color3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Adapter {
    pub hittable: Box<dyn Hittable>,
    pub material: Option<Box<dyn Material>>,
    pub attributes: Attributes,
}

impl Primitive for Adapter {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.attributes.apply(sample, &self.hittable)
    }

    fn bounds(&self) -> Bounds3 {
        todo!()
    }
}
