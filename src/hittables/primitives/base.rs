use crate::util::Bounds3;
use nalgebra::Point3;

pub trait Primitive {
    fn sdf(&self, sample: Point3<f64>) -> f64;
    fn bounds(&self) -> Bounds3;
}
