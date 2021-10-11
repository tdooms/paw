use std::rc::Rc;

use nalgebra::Point3;

use crate::hittables::Hittable;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};

#[derive(Debug, Clone)]
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

    fn material(&self, hit: &Hit) -> Color3 {
        self.objects
            .iter()
            .map(|o| (o, o.sdf(hit.surface)))
            .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
            .unwrap()
            .0
            .material(hit)
    }

    fn bounds(&self) -> Bounds3 {
        // TODO: eh
        Bounds3::infinite()
    }
}
