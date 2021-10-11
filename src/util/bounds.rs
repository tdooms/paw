use nalgebra::{point, vector, Point3, Vector3};

pub struct Bounds3 {
    pub origin: Point3<f64>,
    pub extent: Vector3<f64>,
}

impl Default for Bounds3 {
    fn default() -> Self {
        Self {
            origin: point![0.0, 0.0, 0.0],
            extent: vector![1.0, 1.0, 1.0],
        }
    }
}

impl Bounds3 {
    pub fn infinite() -> Self {
        Self {
            origin: point![0.0, 0.0, 0.0],
            extent: Vector3::from_element(f64::MAX),
        }
    }

    pub fn combine(&self, rhs: &Bounds3) -> Bounds3 {
        // TODO: this is wrong, need to use extent as well
        let min = self
            .origin
            .coords
            .zip_map(&rhs.origin.coords, |a, b| a.min(b));

        let max = self
            .origin
            .coords
            .zip_map(&rhs.origin.coords, |a, b| a.max(b));

        let origin = min.zip_map(&max, |a, b| (a + b) / 2.0).into();
        let extent = min.zip_map(&max, |a, b| (b - a) / 2.0).into();

        Bounds3 { origin, extent }
    }
}
