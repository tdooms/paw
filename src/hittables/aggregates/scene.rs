use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::hittables::Container;
use crate::hittables::Hittable;
use crate::util::Bounds3;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub objects: Vec<Box<dyn Container>>,
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
