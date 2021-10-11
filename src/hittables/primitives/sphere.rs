use crate::hittables::Hittable;
use crate::materials::{Color, Material};
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};
use nalgebra::{vector, Point3};

#[derive(Debug)]
pub struct Sphere {
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            radius: 1.0,
            material: Box::new(Color::default()),
        }
    }
}

impl Hittable for Sphere {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        sample.coords.norm() - self.radius
    }

    fn material(&self, hit: &Hit) -> Color3 {
        self.material.color(hit)
    }

    fn bounds(&self) -> Bounds3 {
        Bounds3::default()
    }
}
