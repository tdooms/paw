use crate::ray::Hit;
use crate::util::{Bounds3, Color3};
use nalgebra::Point3;
use std::fmt::Debug;

pub trait Hittable: Debug {
    fn sdf(&self, sample: Point3<f64>) -> f64;

    fn material(&self, hit: &Hit) -> Color3;

    fn bounds(&self) -> Bounds3;
}
