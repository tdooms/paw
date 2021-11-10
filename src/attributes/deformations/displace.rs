use std::fmt::{Debug, Formatter};

use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::attributes::Attribute;
use crate::hittables::Hittable;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};

#[derive(Debug, Serialize, Deserialize)]
pub struct Displace {
    pub hittable: Box<dyn Hittable>,
    pub scale: f64,
}

// #[typetag::serde(name = "displace")]
// impl Hittable for Displace {
//     fn sdf(&self, sample: Point3<f64>) -> f64 {
//         let d1 = self.hittable.sdf(sample);
//         let d2 = sample
//             .map(|x| (20.0 * x).sin())
//             .coords
//             .fold(1.0, |acc, t| acc * t);
//
//         d1 + d2 * self.scale
//     }
//
//     fn bounds(&self) -> Bounds3 {
//         Bounds3::infinite()
//     }
// }
