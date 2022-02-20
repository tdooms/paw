use nalgebra::{Point3, Vector3};

use crate::config::MarchParams;
use crate::hittables::Hittable;
use crate::lights::PointLight;
use crate::ray::Hit;
use crate::shaders::Shader;
use crate::util::Color3;

struct PathTracer;

impl Shader for PathTracer {
    fn shade(&self, hit: &Hit, eye: Point3<f64>, lights: &[PointLight], world: &dyn Hittable, settings: &MarchParams) -> Color3 {
        
    }
}
