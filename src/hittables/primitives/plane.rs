use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::hittables::Hittable;
use crate::materials::Material;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};

#[derive(Debug, Serialize, Deserialize)]
pub struct Plane {
    pub material: Box<dyn Material>,
}

#[typetag::serde(name = "plane")]
impl Hittable for Plane {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        sample.y
    }

    fn bounds(&self) -> Bounds3 {
        // TODO: eh...
        Bounds3::infinite()
    }
}
