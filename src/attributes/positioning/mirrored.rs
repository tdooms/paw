use nalgebra::{point, Point3};
use serde::{Deserialize, Serialize};

use crate::hittables::Hittable;
use crate::util::Bounds3;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
    XY,
    YZ,
    XYZ,
}

#[derive(Debug)]
pub struct Mirrored {
    pub hittable: Box<dyn Hittable>,
    pub axis: Axis,
}

impl Hittable for Mirrored {
    fn sdf(&self, sample: Point3<f64>) -> f64 {
        let p = sample.coords;
        let sample = match self.axis {
            Axis::X => point![p.x.abs(), p.y, p.z],
            Axis::Y => point![p.x, p.y.abs(), p.z],
            Axis::Z => point![p.x, p.y, p.z.abs()],
            Axis::XY => point![p.x.abs(), p.y.abs(), p.z],
            Axis::YZ => point![p.x, p.y.abs(), p.z.abs()],
            Axis::XYZ => point![p.x.abs(), p.y.abs(), p.z.abs()],
        };
        self.hittable.sdf(sample)
    }

    fn bounds(&self) -> Bounds3 {
        // TODO: fix
        Bounds3::infinite()
    }
}
