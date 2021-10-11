use crate::hittables::Hittable;
use crate::ray::Hit;
use crate::util::{Bounds3, Color3};
use nalgebra::{point, Point3};

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
    XY,
    YZ,
    XYZ,
}

#[derive(Debug, Clone)]
pub struct Mirrored<X: Hittable> {
    pub hittable: X,
    pub axis: Axis,
}

impl<X: Hittable> Hittable for Mirrored<X> {
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

    fn material(&self, hit: &Hit) -> Color3 {
        self.hittable.material(hit)
    }

    fn bounds(&self) -> Bounds3 {
        // TODO: fix
        Bounds3::infinite()
    }
}
