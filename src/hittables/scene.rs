use std::rc::Rc;

use nalgebra::Point3;

pub trait Hittable {
    fn sdf(&self, sample: Point3<f64>) -> f64;
}

pub struct Scene {
    pub objects: Vec<Rc<dyn Hittable>>,
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
}

pub struct Sphere {
    pub origin: Point3<f64>,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        (sample - self.origin).norm() - self.radius
    }
}
