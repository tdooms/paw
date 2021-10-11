use crate::hittables::Hittable;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};
use nalgebra::{Point3, Vector3};

#[derive(Debug, Clone)]
pub struct Translate<X: Hittable> {
    pub hittable: X,
    pub translation: Vector3<f64>,
}

impl<X: Hittable> Hittable for Translate<X> {
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
