use std::f64::consts::FRAC_PI_2;

use bmp::{px, Image, Pixel};
use nalgebra::{point, Point3, Unit, UnitVector3, Vector3};

use crate::camera::Camera;
use crate::hittables::{Hittable, Sphere};
use crate::ray::Ray;

mod camera;
mod hittables;
mod ray;

const EPS: f64 = 0.001;
const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

#[derive(Debug, Clone, Copy)]
struct Hit {
    pub depth: f64,
    pub surface: Point3<f64>,
    pub normal: UnitVector3<f64>,
}

#[derive(Debug, Clone, Copy)]
struct Material {
    ambient: Point3<f64>,
    diffuse: Point3<f64>,
    specular: Point3<f64>,
    shininess: f64,
}

#[derive(Debug, Clone, Copy)]
struct Light {
    position: Point3<f64>,
    color: Point3<f64>,
}

fn estimate_normal(surface: Point3<f64>, scene: &impl Hittable) -> UnitVector3<f64> {
    let x_eps: Vector3<f64> = Vector3::x().scale(EPS);
    let y_eps: Vector3<f64> = Vector3::y().scale(EPS);
    let z_eps: Vector3<f64> = Vector3::z().scale(EPS);

    let x = scene.sdf(surface + x_eps) - scene.sdf(surface - x_eps);
    let y = scene.sdf(surface + y_eps) - scene.sdf(surface - y_eps);
    let z = scene.sdf(surface + z_eps) - scene.sdf(surface - z_eps);

    Unit::new_normalize(Vector3::from([x, y, z]))
}

fn reflect(incident: UnitVector3<f64>, normal: UnitVector3<f64>) -> UnitVector3<f64> {
    let scale = 2.0 * incident.dot(&normal);
    // TODO: this can probably be unchecked
    Unit::new_normalize(incident.into_inner() - normal.scale(scale))
}

fn phong(material: Material, hit: Hit, eye: Point3<f64>, lights: &[Light]) -> Point3<f64> {
    let mut color = material.ambient.coords.scale(0.5); // Why do they divide by two?

    for light in lights {
        let light_dir = Unit::new_normalize(light.position - hit.surface);
        let eye_dir = Unit::new_normalize(eye - hit.surface);
        let reflected = reflect(-light_dir, hit.normal);

        let ln = light_dir.dot(&hit.normal);
        let re = reflected.dot(&eye_dir);

        color += if ln < 0.0 {
            Vector3::default()
        } else if re < 0.0 {
            (light.color.coords.component_mul(&material.diffuse.coords)).scale(ln)
        } else {
            let specular = material.specular.coords.scale(re.powf(material.shininess));
            let diffuse = material.diffuse.coords.scale(ln);
            light.color.coords.component_mul(&(specular + diffuse))
        }
        .component_mul(&light.color.coords);
    }
    Point3::from(color)
}

fn march(ray: Ray, scene: &impl Hittable) -> Option<(Hit)> {
    let mut depth = 0.0;

    for _ in 0..20 {
        let location = ray.at(depth);
        let dist = scene.sdf(location);

        match (depth, dist) {
            (_, 0.0..=0.001) => {
                let hit = Hit {
                    depth,
                    surface: location,
                    normal: estimate_normal(location, scene),
                };
                return Some(hit);
            }
            (1000.0.., _) => return None,
            _ => depth += dist,
        }
    }

    None
}

fn main() {
    let look_from = point![0.0, 0.0, -1.0];
    let look_at = Point3::from([0., 0., 0.]);

    let camera = Camera::new(look_from, look_at, Vector3::y_axis(), FRAC_PI_2, 1.0);

    let lights = vec![Light {
        position: point![4.0, 2.0, -1.0],
        color: point![0.4, 0.4, 0.4],
    }];

    let scene = Sphere {
        origin: Point3::new(0., 0., 1.0),
        radius: 1.0,
    };

    let mut film = Image::new(WIDTH, HEIGHT);

    let material = Material {
        ambient: Point3::from([0.2, 0.2, 0.2]),
        diffuse: Point3::from([0.7, 0.2, 0.2]),
        specular: Point3::from([1.0, 1.0, 1.0]),
        shininess: 10.0,
    };

    for (x, y) in film.coordinates() {
        let ray = camera.ray(x as f64 / WIDTH as f64, y as f64 / HEIGHT as f64);
        let pixel = match march(ray, &scene) {
            None => px!(0, 0, 0),
            Some(hit) => {
                let color = phong(material, hit, look_from, &lights);
                let scaled = color.coords.scale(255.99);
                px!(scaled.x, scaled.y, scaled.z)
            }
        };
        film.set_pixel(x, y, pixel);
    }

    film.save("img.bmp").unwrap()
}
