use nalgebra::{Point3, Vector3};

use crate::config::MarchParams;
use crate::hittables::Hittable;
use crate::lights::PointLight;
use crate::ray::Hit;

pub trait Shader {
    fn shade(
        &self,
        hit: &Hit,
        eye: Point3<f64>,
        lights: &[PointLight],
        world: &dyn Hittable,
        settings: &MarchParams,
    ) -> Vector3<f64>;
}
