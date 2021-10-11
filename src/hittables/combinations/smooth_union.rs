use crate::hittables::Hittable;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};
use nalgebra::Point3;

#[derive(Debug)]
pub struct SmoothUnion {
    pub first: Box<dyn Hittable>,
    pub second: Box<dyn Hittable>,
    pub smoothness: f64,
}

impl SmoothUnion {
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

impl Hittable for SmoothUnion {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let d1 = self.first.sdf(sample);
        let d2 = self.second.sdf(sample);

        let h = (self.smoothness - (d1 - d2).abs()).max(0.0);
        d1.min(d2) - h * h * 0.25 / self.smoothness
    }

    fn material(&self, hit: &Hit) -> Color3 {
        match self.first.sdf(hit.surface) > self.second.sdf(hit.surface) {
            true => self.second.material(hit),
            false => self.first.material(hit),
        }
    }

    fn bounds(&self) -> Bounds3 {
        self.first.bounds().combine(&self.second.bounds())
    }
}
