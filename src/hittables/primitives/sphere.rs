use nalgebra::{vector, Point3};
use serde::{Deserialize, Serialize};

use crate::hittables::Hittable;
use crate::materials::{Color, Material};
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Self { radius: 1.0 }
    }
}

#[typetag::serde(name = "sphere")]
impl Hittable for Sphere {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        sample.coords.norm() - self.radius
    }

    fn bounds(&self) -> Bounds3 {
        Bounds3::default()
    }
}
