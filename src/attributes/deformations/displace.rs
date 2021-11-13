use crate::attributes::Attribute;
use nalgebra::Point3;
use std::fmt::Debug;

use crate::hittables::Hittable;

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Displace(pub f64);

impl Attribute for Displace {
    fn adapt(&self, sample: Point3<f64>, sdf: &dyn Fn(Point3<f64>) -> f64) -> f64 {
        let distortion = sample
            .map(|x| (20.0 * x).sin())
            .coords
            .fold(1.0, |acc, t| acc * t);

        sdf(sample) + distortion * self.0
    }
}
