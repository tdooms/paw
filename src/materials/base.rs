use crate::ray::Hit;
use crate::util::Color3;
use std::fmt::Debug;

pub trait Material: Debug {
    fn color(&self, hit: &Hit) -> Color3;
}
