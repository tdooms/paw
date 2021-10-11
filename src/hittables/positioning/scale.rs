use crate::hittables::Hittable;
use crate::util::Bounds3;
use nalgebra::Point3;

pub struct Scale<X: Hittable> {
    pub hittable: X,
    pub factor: f64,
}

impl<X: Hittable> Hittable for Scale<X> {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let point = sample.coords.scale(1.0 / self.factor).into();
        self.hittable.sdf(point) * self.factor
    }

    fn bounds(&self) -> Bounds3 {
        let inner = self.hittable.bounds();
        Bounds3 {
            origin: inner.origin,
            extent: inner.extent.scale(self.factor),
        }
    }
}
