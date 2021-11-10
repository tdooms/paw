use nalgebra::{vector, Point3, Vector3};
use serde::{Deserialize, Serialize};

use crate::hittables::Hittable;
use crate::util::Bounds3;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cuboid {
    pub extent: Vector3<f64>,
}

impl Hittable for Cuboid {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let d = sample.coords.abs() - vector![1.0, 1.0, 1.0];

        let inside_dist = d.max().min(0.);
        let outside_dist = d.map(|x| x.max(0.)).norm();

        inside_dist + outside_dist
    }

    fn bounds(&self) -> Bounds3 {
        Bounds3::default()
    }
}
