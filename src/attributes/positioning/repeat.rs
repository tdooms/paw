use crate::attributes::Attribute;
use nalgebra::{Point3, Vector3};

use crate::hittables::Hittable;
use crate::util::Bounds3;

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Repeat(pub Vector3<f64>);

impl Attribute for Repeat {
    fn adapt(&self, sample: Point3<f64>, sdf: &dyn Fn(Point3<f64>) -> f64) -> f64 {
        // TODO: this only repeats on positive values, meaning only 1/8 of the space,
        // using .abs() doesn't work
        let t = sample + self.0.scale(0.5);

        let z: Vector3<f64> = t.coords.zip_map(&self.0, |a, b| a % b).into();
        let q = Point3::from(z - self.0.scale(0.5));

        sdf(q)
    }
}

// impl Hittable for Repeat {
//     fn sdf(&self, sample: Point3<f64>) -> f64 {
//
//     }
//
//     fn bounds(&self) -> Bounds3 {
//         Bounds3::infinite()
//     }
// }
