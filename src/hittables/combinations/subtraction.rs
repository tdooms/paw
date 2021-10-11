use crate::hittables::Hittable;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};
use nalgebra::Point3;

#[derive(Debug)]
pub struct Subtraction {
    pub first: Box<dyn Hittable>,
    pub second: Box<dyn Hittable>,
}

impl Subtraction {
    pub fn new(first: impl Hittable + 'static, second: impl Hittable + 'static) -> Self {
        Self {
            first: Box::new(first),
            second: Box::new(second),
        }
    }
}

impl Hittable for Subtraction {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.first.sdf(sample).max(-self.second.sdf(sample))
    }

    fn material(&self, hit: &Hit) -> Color3 {
        self.first.material(hit)
    }

    fn bounds(&self) -> Bounds3 {
        self.first.bounds().combine(&self.second.bounds())
    }
}
