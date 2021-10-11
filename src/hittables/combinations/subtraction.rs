use crate::hittables::Hittable;
use crate::util::Bounds3;
use nalgebra::Point3;

pub struct Subtraction<X: Hittable, Y: Hittable> {
    pub first: X,
    pub second: Y,
}

impl<X: Hittable, Y: Hittable> Hittable for Subtraction<X, Y> {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.first.sdf(sample).max(-self.second.sdf(sample))
    }
    fn bounds(&self) -> Bounds3 {
        self.first.bounds().combine(&self.second.bounds())
    }
}
