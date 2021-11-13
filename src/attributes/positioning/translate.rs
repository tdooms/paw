use nalgebra::{Point3, Vector3};

use crate::attributes::Attribute;

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Translate(Vector3<f64>);

impl Attribute for Translate {
    fn adapt(&self, sample: Point3<f64>, sdf: &dyn Fn(Point3<f64>) -> f64) -> f64 {
        sdf(sample + self.0)
    }
}
