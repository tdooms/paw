use nalgebra::Point3;

use crate::hittables::{Attributes, Hittable, Object};
use crate::materials::Material;
use crate::util::{Bounds3, Color3};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Adapter {
    pub hittable: Box<dyn Object>,
    pub material: Option<Box<dyn Material>>,
    pub attributes: Attributes,
}

impl Hittable for Adapter {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let primitive = |sample| self.hittable.sdf(sample);
        self.attributes.apply(sample, primitive)
    }

    fn bounds(&self) -> Bounds3 {
        todo!()
    }
}
