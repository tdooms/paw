use nalgebra::{Point3, Vector3};

use crate::hittables::Hittable;
use crate::lights::Light;

pub trait Shader {
    fn shade(
        &self,
        surface: Point3<f64>,
        eye: Point3<f64>,
        lights: &[Box<dyn Light>],
        world: &impl Hittable,
    ) -> Vector3<f64>;
}
