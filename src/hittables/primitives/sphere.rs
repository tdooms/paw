use crate::hittables::Hittable;
use crate::util::Bounds3;
use nalgebra::Point3;

pub struct Sphere {
    pub radius: f64,
}

impl Hittable for Sphere {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        sample.coords.norm() - self.radius
    }

    fn bounds(&self) -> Bounds3 {
        Bounds3::default()
    }
}
