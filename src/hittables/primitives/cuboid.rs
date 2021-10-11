use crate::hittables::Hittable;
use crate::util::Bounds3;
use nalgebra::{Point3, Vector3};

pub struct Cuboid {
    pub extent: Vector3<f64>,
}

impl Hittable for Cuboid {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let d = sample.coords.abs() - self.extent;

        let inside_dist = d.max().min(0.);
        let outside_dist = d.map(|x| x.max(0.)).norm();

        inside_dist + outside_dist
    }

    fn bounds(&self) -> Bounds3 {
        Bounds3::default()
    }
}