use crate::util::Bounds3;
use nalgebra::Point3;

pub trait Hittable {
    fn sdf(&self, sample: Point3<f64>) -> f64;

    fn bounds(&self) -> Bounds3;
}
