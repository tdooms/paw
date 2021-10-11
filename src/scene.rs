use crate::hittables::{Cuboid, Plane, Scene, SmoothSubtraction, Sphere, Translate};
use std::rc::Rc;

use nalgebra::{point, vector};

use crate::hittables::Hittable;
use crate::lights::{Light, PointLight};

pub fn create_world() -> impl Hittable {
    let h1 = SmoothSubtraction {
        first: Sphere { radius: 1.1 },
        second: Translate {
            hittable: Cuboid {
                extent: vector![1.0, 1.0, 1.0],
            },
            translation: vector![0.1, 0.0, 0.0],
        },
        smoothness: 0.25,
    };

    let h2 = Translate {
        hittable: Plane,
        translation: vector![0.0, 1.0, 0.0],
    };

    Scene {
        objects: vec![Rc::new(h1), Rc::new(h2)],
    }
}

pub fn create_lights() -> Vec<Box<dyn Light>> {
    let light = PointLight {
        position: point![4.0, 2.0, -1.0],
        color: point![0.4, 0.4, 0.4],
        softness: 8.0,
    };

    vec![Box::new(light)]
}
