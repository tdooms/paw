use nalgebra::Point3;

use crate::Hit;
use crate::hittables::{Attributes, Hittable};
use crate::materials::Material;
use crate::util::{Bounds3, Color3};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Union {
    pub first: Box<dyn Hittable>,
    pub second: Box<dyn Hittable>,
    pub smoothness: Option<f64>,

    #[serde(flatten)]
    pub attributes: Attributes,
    pub material: Option<Box<dyn Material>>,
}

impl Union {
    fn inner(&self, sample: Point3<f64>) -> f64 {
        let d1 = self.first.sdf(sample);
        let d2 = self.second.sdf(sample);

        if let Some(smoothness) = self.smoothness {
            let h = (smoothness - (d1 - d2).abs()).max(0.0);
            d1.min(d2) - h * h * 0.25 / smoothness
        } else {
            d1.min(d2)
        }
    }
}

#[typetag::serde(name = "union")]
impl Hittable for Union {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.attributes.apply(sample, |sample| self.inner(sample))
    }

    fn bounds(&self) -> Bounds3 {
        // self.first.bounds().combine(&self.second.bounds())
        Bounds3::infinite()
    }

    fn color(&self, hit: &Hit) -> Color3 {
        // TODO: this is wrong
        match &self.material {
            Some(material) => material.color(hit),
            None if self.first.sdf(hit.surface) > self.second.sdf(hit.surface) => {
                self.second.color(hit)
            }
            _ => self.first.color(hit),
        }
    }
}
