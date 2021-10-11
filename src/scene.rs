use std::rc::Rc;

use nalgebra::{point, vector};

use crate::hittables::Hittable;
use crate::hittables::{Cuboid, Plane, Scene, SmoothSubtraction, Sphere, Translate};
use crate::lights::{Light, PointLight};
use crate::materials::{Checkered, Color};

pub fn create_world() -> impl Hittable {
    let cube = Translate {
        hittable: Cuboid {
            extent: vector![1.0, 1.0, 1.0],
            material: Box::new(Color {
                color: vector![0.1, 0.8, 0.1],
            }),
        },
        translation: vector![0.1, 0.0, 0.0],
    };

    let h1 = SmoothSubtraction::new(
        Sphere {
            radius: 1.1,
            material: Box::new(Color {
                color: vector![0.1, 0.2, 0.7],
            }),
        },
        cube,
        0.25,
    );

    let h2 = Translate {
        hittable: Plane {
            material: Box::new(Checkered {
                c1: vector![1.0, 1.0, 1.0],
                c2: vector![0.2, 0.2, 0.2],
                repeat: 0.3,
            }),
        },
        translation: vector![0.0, 1.0, 0.0],
    };

    Scene {
        objects: vec![Rc::new(h1), Rc::new(h2)],
    }
}

pub fn create_lights() -> Vec<PointLight> {
    let light = PointLight {
        position: point![4.0, 2.0, -1.0],
        color: vector![0.4, 0.4, 0.4],
        softness: 8.0,
    };

    vec![light]
}
