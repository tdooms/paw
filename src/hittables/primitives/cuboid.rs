use std::rc::Rc;

use nalgebra::{Point3, vector, Vector3};
use serde::{Deserialize, Serialize};

use crate::hittables::Hittable;
use crate::materials::{Color, Material};
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cuboid {
    pub extent: Vector3<f64>,
}

impl Default for Cuboid {
    fn default() -> Self {
        Self {
            extent: vector![1.0, 1.0, 1.0],
        }
    }
}

#[typetag::serde(name = "cuboid")]
impl Hittable for Cuboid {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let d = sample.coords.abs() - self.extent;

        let inside_dist = d.max().min(0.);
        let outside_dist = d.map(|x| x.max(0.)).norm();

        inside_dist + outside_dist
    }

    fn bounds(&self) -> Bounds3 {
        Bounds3::default()
    }
}
