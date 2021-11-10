use crate::util::Bounds3;
use nalgebra::Point3;

pub trait Hittable: Primitive {
    fn color(&self, sample: Point3<f64>) -> f64;
}
