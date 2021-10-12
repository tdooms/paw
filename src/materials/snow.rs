use crate::materials::Material;
use crate::ray::Hit;
use crate::util::Color3;
use nalgebra::{vector, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Snow;

#[typetag::serde(name = "snow")]
impl Material for Snow {
    fn color(&self, hit: &Hit) -> Color3 {
        let angle = hit.normal.dot(&Vector3::y_axis());

        let snow = vector![1.0, 1.0, 1.0];
        let dirt = vector![0.58, 0.27, 0.07];

        if angle > 0.7 {
            snow
        } else if angle > 0.5 {
            let alpha = (angle - 0.5) * 5.0;
            dirt.lerp(&snow, alpha)
        } else {
            dirt
        }
    }
}
