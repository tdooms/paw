use crate::config::Settings;
use nalgebra::{Point3, Unit, UnitVector3, Vector3};

use crate::hittables::Hittable;
use crate::util::tetrahedron_normal;

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub depth: f64,
    pub surface: Point3<f64>,
    pub normal: UnitVector3<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3<f64>,
    direction: UnitVector3<f64>,
}

impl Ray {
    pub fn to(from: Point3<f64>, to: Point3<f64>) -> Self {
        Self {
            origin: from,
            direction: Unit::new_normalize(to - from),
        }
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction.scale(t)
    }

    pub fn march(
        &self,
        world: &dyn Hittable,
        eye: Point3<f64>,
        settings: &Settings,
    ) -> Option<Hit> {
        let mut depth = settings.start_eps;

        for _ in 0..settings.max_steps {
            let location = self.at(depth);
            let dist = world.sdf(location);

            if dist < settings.hit_eps {
                let hit = Hit {
                    depth,
                    surface: location,
                    normal: tetrahedron_normal(eye, location, world),
                };
                return Some(hit);
            }

            depth += dist;

            if depth > settings.max_t {
                return None;
            }
        }
        None
    }

    // https://www.iquilezles.org/www/articles/rmshadows/rmshadows.htm
    pub fn closest(
        from: Point3<f64>,
        to: Point3<f64>,
        world: &dyn Hittable,
        settings: &Settings,
        smoothness: f64,
    ) -> f64 {
        let ray = Ray::to(from, to);
        let max = (from - to).norm();

        let mut depth = settings.start_eps;
        let mut res = 1.0f64;
        let mut ph = 1e20f64;

        while depth < max {
            let dist = world.sdf(ray.at(depth));

            if dist < settings.hit_eps {
                return 0.0;
            }

            let squared = dist * dist;
            let y = squared / (2.0 * ph);
            let d = (squared - y * y).sqrt();

            res = res.min(smoothness * d / 0.0f64.max(depth - y));
            ph = depth;
            depth += dist;
        }
        res
    }
}
