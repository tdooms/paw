use nalgebra::{Point3, vector};
use pawcro::primitive;

#[primitive(Cube)]
fn cube(sample: Point3<f64>) -> f64 {
    let d = sample.coords.abs() - vector![1.0, 1.0, 1.0];

    let inside_dist = d.max().min(0.);
    let outside_dist = d.map(|x| x.max(0.)).norm();

    inside_dist + outside_dist
}
