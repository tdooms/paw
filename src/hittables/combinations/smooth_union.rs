use crate::hittables::Hittable;
use crate::util::Bounds3;
use nalgebra::Point3;

pub struct SmoothUnion<X: Hittable, Y: Hittable> {
    pub first: X,
    pub second: Y,
    pub smoothness: f64,
}

impl<X: Hittable, Y: Hittable> Hittable for SmoothUnion<X, Y> {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let d1 = self.first.sdf(sample);
        let d2 = self.second.sdf(sample);

        let h = (self.smoothness - (d1 - d2).abs()).max(0.0);
        d1.min(d2) - h * h * 0.25 / self.smoothness
    }
    fn bounds(&self) -> Bounds3 {
        self.first.bounds().combine(&self.second.bounds())
    }
}
