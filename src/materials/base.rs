use std::fmt::Debug;

use crate::ray::Hit;
use crate::util::Color3;

#[typetag::serde(tag = "type")]
pub trait Material: Debug + Send + Sync {
    fn color(&self, hit: &Hit) -> Color3;
}
