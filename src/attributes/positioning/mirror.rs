use nalgebra::{point, Point3};

use crate::attributes::Attribute;

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
    XY,
    YZ,
    XYZ,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Mirror(pub Axis);

impl Attribute for Mirror {
    fn adapt(&self, sample: Point3<f64>, sdf: &dyn Fn(Point3<f64>) -> f64) -> f64 {
        let p = sample.coords;
        let sample = match self.0 {
            Axis::X => point![p.x.abs(), p.y, p.z],
            Axis::Y => point![p.x, p.y.abs(), p.z],
            Axis::Z => point![p.x, p.y, p.z.abs()],
            Axis::XY => point![p.x.abs(), p.y.abs(), p.z],
            Axis::YZ => point![p.x, p.y.abs(), p.z.abs()],
            Axis::XYZ => point![p.x.abs(), p.y.abs(), p.z.abs()],
        };
        sdf(sample)
    }
}
