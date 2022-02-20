use nalgebra::Point3;

use crate::attributes::*;

// TODO: change this to something faster and more extensible
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Attributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elongate: Option<Elongate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onion: Option<Onion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round: Option<Round>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displace: Option<Displace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror: Option<Mirror>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<Repeat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Scale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translate: Option<Translate>,
}

impl Attributes {
    fn modify<T: Attribute + 'static>(
        attribute: Option<T>,
        previous: impl Fn(Point3<f64>) -> f64,
    ) -> impl Fn(Point3<f64>) -> f64 {
        move |sample| match attribute {
            Some(attribute) => attribute.adapt(sample, &previous),
            None => previous(sample),
        }
    }

    pub fn apply(&self, sample: Point3<f64>, sdf: impl Fn(Point3<f64>) -> f64) -> f64 {
        let scaled = Self::modify(self.scale, &sdf);
        let translated = Self::modify(self.translate, &scaled);
        let displaced = Self::modify(self.displace, &translated);
        let mirrored = Self::modify(self.mirror, &displaced);
        let repeated = Self::modify(self.repeat, &mirrored);

        repeated(sample)
    }
}
