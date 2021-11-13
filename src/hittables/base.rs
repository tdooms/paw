use std::fmt::Debug;

use nalgebra::Point3;

use crate::util::{Bounds3, Color3};
use crate::Hit;

#[typetag::serde(tag = "type")]
pub trait Hittable: Debug {
    fn sdf(&self, sample: Point3<f64>) -> f64;
    fn bounds(&self) -> Bounds3;
    fn color(&self, hit: &Hit) -> Color3;
}
