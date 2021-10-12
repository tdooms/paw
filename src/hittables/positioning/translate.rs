use crate::hittables::Hittable;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};
use nalgebra::{Point3, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Translate {
    pub hittable: Box<dyn Hittable>,
    pub translation: Vector3<f64>,
}

#[typetag::serde(name = "translate")]
impl Hittable for Translate {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.hittable.sdf(sample + self.translation)
    }

    fn material(&self, hit: &Hit) -> Color3 {
        self.hittable.material(hit)
    }

    fn bounds(&self) -> Bounds3 {
        let inner = self.hittable.bounds();
        Bounds3 {
            origin: inner.origin + self.translation,
            extent: inner.extent,
        }
    }
}
