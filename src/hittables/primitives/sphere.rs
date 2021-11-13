use nalgebra::Point3;
use pawcro::primitive;

#[primitive(Sphere)]
fn sphere(sample: Point3<f64>) -> f64 {
    sample.coords.norm() - 1.0
}
