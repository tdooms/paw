use super::Hittable;
use nalgebra::Point3;

pub struct Sphere {
    pub origin: Point3<f64>,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        (sample - self.origin).norm() - self.radius
    }
}
