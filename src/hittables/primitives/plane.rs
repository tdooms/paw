use crate::hittables::Hittable;
use crate::materials::Material;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};
use nalgebra::Point3;

#[derive(Debug)]
pub struct Plane {
    pub material: Box<dyn Material>,
}

impl Hittable for Plane {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        sample.y
    }

    fn material(&self, hit: &Hit) -> Color3 {
        self.material.color(hit)
    }

    fn bounds(&self) -> Bounds3 {
        // TODO: eh...
        Bounds3::infinite()
    }
}
