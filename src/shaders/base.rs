use nalgebra::{Point3, Vector3};

use crate::config::{MarchParams, Config};
use crate::hittables::Hittable;
use crate::lights::PointLight;
use crate::ray::Hit;

pub trait Shader {
    fn shade(
        &self, hit: &Hit, from: Point3<f64>, config: &Config
    ) -> Vector3<f64>;
}
