use nalgebra::Point3;

pub trait Hittable {
    fn sdf(&self, sample: Point3<f64>) -> f64;
}
