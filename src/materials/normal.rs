use crate::materials::Material;
use crate::ray::Hit;
use crate::util::Color3;
use nalgebra::{vector, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Normal;

#[typetag::serde(name = "normal")]
impl Material for Normal {
    fn color(&self, hit: &Hit) -> Color3 {
        hit.normal.into_inner()
    }
}