use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::hittables::{Hittable, Object};
use crate::materials::Material;
use crate::util::Bounds3;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
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

    fn bounds(&self) -> Bounds3 {
        // TODO: eh
        Bounds3::infinite()
    }
}
