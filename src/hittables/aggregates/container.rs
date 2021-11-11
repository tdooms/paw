use nalgebra::Point3;

use crate::hittables::{Attributes, Hittable, Object};
use crate::materials::Material;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Container {
    pub primitive: Box<dyn Hittable>,
    pub material: Box<dyn Material>,
    pub attributes: Attributes,
}

impl Hittable for Container {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let primitive = |sample| self.primitive.sdf(sample);
        self.attributes.apply(sample, primitive)
    }

    fn bounds(&self) -> Bounds3 {
        self.primitive.bounds()
    }
}

impl Material for Container {
    fn color(&self, hit: &Hit) -> Color3 {
        self.material.color(hit)
    }
}

impl Object for Container {}
