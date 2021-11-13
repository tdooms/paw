use nalgebra::Point3;
use pawcro::primitive;

#[primitive(Plane)]
fn plane(sample: Point3<f64>) -> f64 {
    sample.y
}
