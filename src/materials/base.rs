use crate::ray::Hit;
use crate::util::Color3;
use serde::Deserialize;
use std::fmt::Debug;

#[typetag::serde]
pub trait Material: Debug {
    fn color(&self, hit: &Hit) -> Color3;
}
