use nalgebra::Point3;

use crate::attributes::Attribute;

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Scale(pub f64);

impl Attribute for Scale {
    fn adapt(&self, sample: Point3<f64>, sdf: &dyn Fn(Point3<f64>) -> f64) -> f64 {
        let point = sample.coords.scale(1.0 / self.0).into();
        sdf(point) * self.0
    }
}
