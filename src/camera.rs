use nalgebra::{Point3, Unit, UnitVector3, Vector3};

use crate::ray::Ray;

pub struct Camera {
    origin: Point3<f64>,
    lower_left: Point3<f64>,

    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new(
        look_from: Point3<f64>,
        look_at: Point3<f64>,
        up: UnitVector3<f64>,
        fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let viewport_height = (fov / 2.0).tan() * 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        // The depth direction, aka Z-axis
        let straight = Unit::new_normalize(look_from - look_at);
        // The Y-axis computed by cross product of Z and X axis.
        let left = straight.cross(&up);
        // The X-axis is not the same as the up vector.
        let above = straight.cross(&left);

        let horizontal = left.scale(viewport_width);
        let vertical = above.scale(viewport_height);

        let lower_left =
            Point3::from(look_from - straight.into_inner() - horizontal / 2.0 - vertical / 2.0);
        let origin = Point3::from(look_from);

        Self {
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, x: f64, y: f64) -> Ray {
        assert!(x >= 0. && x <= 1. && y >= 0. && y <= 1.);
        Ray::to(
            self.origin,
            self.lower_left + self.horizontal.scale(x) + self.vertical.scale(y),
        )
    }
}
