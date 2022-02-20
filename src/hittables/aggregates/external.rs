use std::fmt;
use std::fs::read_to_string;

use nalgebra::Point3;
use serde::{Deserializer, Serializer};
use serde::de::{self, EnumAccess, Error, MapAccess, SeqAccess, Visitor};

use crate::Hit;
use crate::hittables::{Attributes, Hittable};
use crate::util::{Bounds3, Color3};

struct PathVisitor;

impl<'de> Visitor<'de> for PathVisitor {
    type Value = ExternalInfo;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        // TODO: remove unwraps
        let contents = read_to_string(format!("scenes/external/{v}")).unwrap();
        Ok(ExternalInfo {
            world: serde_json::from_str(&contents).unwrap(),
            location: v.to_string(),
        })
    }
}

fn deserialize<'de, D>(d: D) -> Result<ExternalInfo, D::Error> where D: Deserializer<'de> {
    d.deserialize_str(PathVisitor)
}

fn serialize<S>(v: &ExternalInfo, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(&v.location)
}

struct ExternalInfo {
    world: Box<dyn Hittable>,
    location: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct External {
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    #[serde(rename = "path")]
    pub external: Box<dyn Hittable>,

    #[serde(flatten)]
    pub attributes: Attributes,
}

#[typetag::serde(name = "external")]
impl Hittable for External {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        self.attributes.apply(sample, |sample| self.external.sdf(sample))
    }

    fn bounds(&self) -> Bounds3 {
        // TODO: eh
        Bounds3::infinite()
    }

    fn color(&self, hit: &Hit) -> Color3 {
        self.external.color(hit)
    }
}
