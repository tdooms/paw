use nalgebra::Point3;

pub trait Light {
    fn sample(&self, point: Point3<f64>) -> f64;
}
