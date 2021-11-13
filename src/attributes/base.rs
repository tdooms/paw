use std::fmt::Debug;

use nalgebra::Point3;

pub trait Attribute: Debug + Copy {
    fn adapt(&self, sample: Point3<f64>, sdf: &dyn Fn(Point3<f64>) -> f64) -> f64;
}
