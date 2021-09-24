use super::Hittable;
use nalgebra::Point3;
use std::rc::Rc;

pub struct Union {
    pub first: Rc<dyn Hittable>,
    pub second: Rc<dyn Hittable>,
}

impl Hittable for Union {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.first.sdf(sample).min(self.second.sdf(sample))
    }
}
