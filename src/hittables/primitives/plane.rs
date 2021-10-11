use crate::hittables::Hittable;
use crate::util::Bounds3;
use nalgebra::Point3;

pub struct Plane;

impl Hittable for Plane {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        sample.y
    }

    fn bounds(&self) -> Bounds3 {
        // TODO: eh...
        Bounds3::infinite()
    }
}
