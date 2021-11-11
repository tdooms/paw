use crate::attributes::*;
use crate::hittables::Hittable;
use crate::util::Color3;
use nalgebra::Point3;

#[derive(Debug)]
pub struct Attributes {
    pub elongate: Option<Elongate>,
    pub onion: Option<Onion>,
    pub round: Option<Round>,
    pub displace: Option<Displace>,
    pub mirrored: Option<Mirrored>,
    pub repeat: Option<Repeat>,
    pub scale: Option<Scale>,
    pub translate: Option<Translate>,
}

impl Attributes {
    pub fn apply(&self, sample: Point3<f64>, primitive: impl Fn(Point3<f64>) -> f64) -> f64 {
        let scaled = |sample| match self.scale.as_ref() {
            Some(scale) => scale.adapt(sample, &primitive),
            None => primitive(sample),
        };

        let translated = |sample| match self.translate.as_ref() {
            Some(translate) => translate.adapt(sample, &scaled),
            None => scaled(sample),
        };

        translated(sample)
    }
}
