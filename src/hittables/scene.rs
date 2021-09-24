use std::rc::Rc;

use super::Hittable;
use nalgebra::Point3;

pub struct Scene {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl Hittable for Scene {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        // TODO: safe unwrap and such to avoid NaN
        self.objects
            .iter()
            .map(|o| o.sdf(sample))
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or(0.)
    }
}
