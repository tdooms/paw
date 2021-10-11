use nalgebra::{vector, Point3, Vector3};
use std::rc::Rc;

use crate::hittables::Hittable;
use crate::materials::{Color, Material};
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};

#[derive(Debug)]
pub struct Cuboid {
    pub extent: Vector3<f64>,
    pub material: Box<dyn Material>,
}

impl Default for Cuboid {
    fn default() -> Self {
        Self {
            extent: vector![1.0, 1.0, 1.0],
            material: Box::new(Color::default()),
        }
    }
}

impl Hittable for Cuboid {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let d = sample.coords.abs() - self.extent;

        let inside_dist = d.max().min(0.);
        let outside_dist = d.map(|x| x.max(0.)).norm();

        inside_dist + outside_dist
    }

    fn material(&self, hit: &Hit) -> Color3 {
        self.material.color(hit)
    }

    fn bounds(&self) -> Bounds3 {
        Bounds3::default()
    }
}
