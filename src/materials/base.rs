use std::fmt::Debug;

use serde::Deserialize;

use crate::ray::Hit;
use crate::util::Color3;

#[typetag::serde]
pub trait Material: Debug {
    fn color(&self, hit: &Hit) -> Color3;
}
