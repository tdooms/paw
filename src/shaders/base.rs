use crate::config::Settings;
use nalgebra::{Point3, Vector3};

use crate::hittables::Hittable;
use crate::lights::{Light, PointLight};
use crate::ray::Hit;

pub trait Shader {
    fn shade(
        &self,
        hit: &Hit,
        eye: Point3<f64>,
        lights: &[PointLight],
        world: &dyn Hittable,
        settings: &Settings,
    ) -> Vector3<f64>;
}
