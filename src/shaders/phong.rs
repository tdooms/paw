use nalgebra::{Point3, Unit, vector, Vector3};

use crate::config::{MarchParams, Config};
use crate::hittables::Hittable;
use crate::lights::PointLight;
use crate::ray::{Hit, Ray};
use crate::shaders::base::Shader;
use crate::util::{Color3, reflect};
use std::sync::Arc;

pub struct Phong;

impl Shader for Phong {
    fn shade(&self, hit: &Hit, from: Point3<f64>, config: &Config) -> Color3 {
        let ambient = vector![0.2, 0.2, 0.2];
        let diffuse = world.color(hit);
        let specular = vector![1.0, 1.0, 1.0];
        let shininess = 10.0;

        let mut color = ambient;
        let eye = config.camera.look_from;

        for light in &config.lights {
            let light_dir = Unit::new_normalize(light.position - hit.surface);
            let eye_dir = Unit::new_normalize(eye - hit.surface);
            let reflected = reflect(-light_dir, hit.normal);

            let ln = light_dir.dot(&hit.normal);
            let re = reflected.dot(&eye_dir);

            let shadow = Ray::closest(hit.surface, light.position, world, settings, light.softness);
            let light_contrib = light.color.scale(shadow);

            color += if ln < 0.0 {
                Vector3::default()
            } else if re < 0.0 {
                diffuse.scale(ln).component_mul(&light_contrib)
            } else {
                (specular.scale(re.powf(shininess)) + diffuse.scale(ln))
                    .component_mul(&light_contrib)
            }
        }

        color
    }
}
