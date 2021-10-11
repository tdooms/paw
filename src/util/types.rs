use crate::hittables::Hittable;
use nalgebra::Vector3;

pub type Color3 = Vector3<f64>;

pub type World = Box<dyn Hittable>;
