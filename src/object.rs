use nalgebra::{Point3, Vector3};
use serde::{Deserialize, Serialize};

use crate::attributes::*;
use crate::hittables::Hittable;
use crate::materials::Material;
use crate::util::Bounds3;

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub hittable: Box<dyn Hittable>,
    pub material: Box<dyn Material>,

    // Attributes of the object
    pub elongate: Option<Elongate>,
    pub onion: Option<Onion>,
    pub round: Option<Round>,
    pub displace: Option<Displace>,
    pub mirrored: Option<Mirrored>,
    pub repeat: Option<Repeat>,
    pub scale: Option<Scale>,
    pub translate: Option<Translate>,
}

impl Object {
    pub fn sdf(&self, mut sample: Point3<f64>) -> f64 {
        let primitive = |sample| self.hittable.sdf(sample);

        let scaled = |sample| match self.scale.as_ref() {
            Some(scale) => scale.adapt(sample, &primitive),
            None => primitive(sample)
        };

        let translated = |sample| match self.translate.as_ref() {
            Some(translate) => translate.adapt(sample, &scaled),
            None => scaled(sample)
        };

        translated(sample)
    }
}
