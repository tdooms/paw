use super::Hittable;
use nalgebra::{Point3, Vector3};

pub struct Sphere {
    pub origin: Point3<f64>,
}

impl Hittable for Sphere {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let d = sample.coords.abs() - Vector3::new(1., 1., 1.);

        let inside_dist = d.max().min(0.);
        let outside_dist = d.map(|x| x.max(0.)).norm();

        inside_dist + outside_dist
    }
}
