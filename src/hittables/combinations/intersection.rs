use nalgebra::Point3;

use crate::hittables::Hittable;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};

#[derive(Debug)]
pub struct Intersection {
    pub first: Box<dyn Hittable>,
    pub second: Box<dyn Hittable>,
}

impl Intersection {
    pub fn new(first: impl Hittable + 'static, second: impl Hittable + 'static) -> Self {
        Self {
            first: Box::new(first),
            second: Box::new(second),
        }
    }
}

impl Hittable for Intersection {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.first.sdf(sample).max(self.second.sdf(sample))
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
