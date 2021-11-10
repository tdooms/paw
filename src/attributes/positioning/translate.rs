use nalgebra::{Point3, Vector3};
use serde::{Deserialize, Serialize};

use crate::attributes::Attribute;

#[derive(Debug, Serialize, Deserialize)]
pub struct Translate(Vector3<f64>);

impl Attribute for Translate {
    fn adapt(&self, sample: Point3<f64>, primitive: &dyn Fn(Point3<f64>) -> f64) -> f64 {
        primitive(sample + self.0)
    }
}
