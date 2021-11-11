use nalgebra::{vector, Point3, Unit, UnitVector3, Vector3};

use crate::hittables::Object;

pub fn naive_normal(
    eye: Point3<f64>,
    surface: Point3<f64>,
    world: &impl Object,
) -> UnitVector3<f64> {
    // Scale up the normal polling offset when far away to improve aliasing
    let h = 0.001 * (surface - eye).norm();
    let k = vector![h, 0.0];

    let x = world.sdf(surface + k.xyy()) - world.sdf(surface - k.xyy());
    let y = world.sdf(surface + k.yxy()) - world.sdf(surface - k.yxy());
    let z = world.sdf(surface + k.yyx()) - world.sdf(surface - k.yyx());

    Unit::new_normalize(Vector3::from([x, y, z]))
}

pub fn tetrahedron_normal(
    eye: Point3<f64>,
    surface: Point3<f64>,
    world: &dyn Object,
) -> UnitVector3<f64> {
    // Scale up the normal polling offset when far away to improve aliasing
    let h = 0.001 * (surface - eye).norm();
    let k = vector![1.0, -1.0];
    let kh = vector![h, -h];

    let normal = k.xyy().scale(world.sdf(surface + kh.xyy()))
        + k.yyx().scale(world.sdf(surface + kh.yyx()))
        + k.yxy().scale(world.sdf(surface + kh.yxy()))
        + k.xxx().scale(world.sdf(surface + kh.xxx()));

    Unit::new_normalize(normal)
}
