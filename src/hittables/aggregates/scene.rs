use nalgebra::Point3;

use crate::hittables::{Attributes, Hittable};
use crate::util::{Bounds3, Color3};
use crate::Hit;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,

    #[serde(flatten)]
    pub attributes: Attributes,
}

impl Scene {
    fn inner(&self, sample: Point3<f64>) -> f64 {
        // TODO: safe unwrap and such to avoid NaN
        self.objects
            .iter()
            .map(|o| o.sdf(sample))
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or(0.)
    }
}

#[typetag::serde(name = "scene")]
impl Hittable for Scene {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.attributes.apply(sample, |sample| self.inner(sample))
    }

    fn bounds(&self) -> Bounds3 {
        // TODO: eh
        Bounds3::infinite()
    }

    fn color(&self, _hit: &Hit) -> Color3 {
        todo!()
    }
}
