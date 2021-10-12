use crate::hittables::Hittable;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};
use nalgebra::Point3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale {
    pub hittable: Box<dyn Hittable>,
    pub factor: f64,
}

#[typetag::serde(name = "scale")]
impl Hittable for Scale {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let point = sample.coords.scale(1.0 / self.factor).into();
        self.hittable.sdf(point) * self.factor
    }

    fn material(&self, hit: &Hit) -> Color3 {
        self.hittable.material(hit)
    }

    fn bounds(&self) -> Bounds3 {
        let inner = self.hittable.bounds();
        Bounds3 {
            origin: inner.origin,
            extent: inner.extent.scale(self.factor),
        }
    }
}
