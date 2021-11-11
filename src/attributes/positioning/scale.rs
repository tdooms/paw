use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::attributes::Attribute;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale(pub f64);

impl Attribute for Scale {
    fn adapt(&self, sample: Point3<f64>, primitive: &dyn Fn(Point3<f64>) -> f64) -> f64 {
        let point = sample.coords.scale(1.0 / self.0).into();
        primitive(point) * self.0
    }
}
