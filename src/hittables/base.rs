use crate::materials::Material;
use crate::util::Bounds3;
use nalgebra::Point3;
use std::fmt::Debug;

pub trait Hittable: Debug {
    fn sdf(&self, sample: Point3<f64>) -> f64;
    fn bounds(&self) -> Bounds3;
}

pub trait Object: Hittable + Material {}
