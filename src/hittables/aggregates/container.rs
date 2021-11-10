use nalgebra::Point3;

use crate::hittables::base::Hittable;
use crate::hittables::{Attributes, Container};
use crate::materials::Material;
use crate::util::{Bounds3, Color3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    pub primitive: Box<dyn Primitive>,
    pub material: Box<dyn Material>,
    pub attributes: Attributes,
}

impl Primitive for Container {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.attributes.apply(sample, &self.primitive)
    }

    fn bounds(&self) -> Bounds3 {
        Bounds3::infinite()
    }
}
