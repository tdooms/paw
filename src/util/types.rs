use crate::hittables::Hittable;
use crate::ray::Hit;
use crate::util::Bounds3;
use nalgebra::{Point3, Vector3};

pub type Color3 = Vector3<f64>;

pub type World = Box<dyn Hittable>;

// Nasty workaround for box impl
impl<T: Hittable + ?Sized> Hittable for Box<T> {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        (**self).sdf(sample)
    }

    fn material(&self, hit: &Hit) -> Color3 {
        (**self).material(hit)
    }

    fn bounds(&self) -> Bounds3 {
        (**self).bounds()
    }
}
