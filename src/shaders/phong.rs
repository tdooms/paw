use nalgebra::{point, Point3, Unit, Vector3};

use crate::hittables::Hittable;
use crate::lights::Light;
use crate::ray::Ray;
use crate::shaders::base::Shader;
use crate::util::{reflect, tetrahedron_normal, Color3};

pub struct Phong;

impl Shader for Phong {
    fn shade(
        &self,
        surface: Point3<f64>,
        eye: Point3<f64>,
        lights: &[Box<dyn Light>],
        world: &impl Hittable,
    ) -> Color3 {
        let ambient = point![0.2, 0.2, 0.2];
        let diffuse = point![0.7, 0.2, 0.2];
        let specular = point![1.0, 1.0, 1.0];
        let shininess = 10.0;

        let normal = tetrahedron_normal(eye, surface, world);

        let mut color = ambient;

        for light in lights {
            let light_dir = Unit::new_normalize(light.position - surface);
            let eye_dir = Unit::new_normalize(eye - surface);
            let reflected = reflect(-light_dir, normal);

            let ln = light_dir.dot(&normal);
            let re = reflected.dot(&eye_dir);

            color += if ln < 0.0 {
                Vector3::default()
            } else if re < 0.0 {
                (light.color.coords.component_mul(&diffuse.coords)).scale(ln)
            } else {
                specular.coords.scale(re.powf(shininess)) + diffuse.coords.scale(ln)
            }
            .component_mul(&light.color.coords);
        }

        color
    }
}
