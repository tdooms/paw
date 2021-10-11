use crate::hittables::Hittable;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};
use nalgebra::Point3;

#[derive(Debug)]
pub struct SmoothIntersection {
    pub first: Box<dyn Hittable>,
    pub second: Box<dyn Hittable>,
    pub smoothness: f64,
}

impl SmoothIntersection {
    pub fn new(
        first: impl Hittable + 'static,
        second: impl Hittable + 'static,
        smoothness: f64,
    ) -> Self {
        Self {
            first: Box::new(first),
            second: Box::new(second),
            smoothness,
        }
    }
}

impl Hittable for SmoothIntersection {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let d1 = self.first.sdf(sample);
        let d2 = self.second.sdf(sample);

        let h = (self.smoothness - (d1 - d2).abs()).max(0.0);
        d1.max(d2) + h * h * 0.25 / self.smoothness
    }

    fn material(&self, hit: &Hit) -> Color3 {
        self.first.material(hit)
    }

    fn bounds(&self) -> Bounds3 {
        self.first.bounds().combine(&self.second.bounds())
    }
}
