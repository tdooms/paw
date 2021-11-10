use std::fmt::Debug;

use nalgebra::Point3;

pub trait Attribute: Debug {
    fn adapt(&self, sample: Point3<f64>, primitive: &dyn Fn(Point3<f64>) -> f64) -> f64;
}
