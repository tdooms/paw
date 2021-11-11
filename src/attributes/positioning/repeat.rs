use nalgebra::{Point3, Vector3};
use serde::{Deserialize, Serialize};

use crate::hittables::Hittable;
use crate::util::Bounds3;

#[derive(Debug)]
pub struct Repeat {
    pub hittable: Box<dyn Hittable>,
    pub repetition: Vector3<f64>,
}

impl Hittable for Repeat {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        // TODO: this only repeats on positive values, meaning only 1/8 of the space,
        // using .abs() doesn't work
        let c = self.repetition;
        let t = sample + c.scale(0.5);

        let z: Vector3<f64> = t.coords.zip_map(&c, |a, b| a % b).into();
        let q = Point3::from(z - c.scale(0.5));

        self.hittable.sdf(q)
    }

    fn bounds(&self) -> Bounds3 {
        Bounds3::infinite()
    }
}
