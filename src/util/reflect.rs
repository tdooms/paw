use nalgebra::{Unit, UnitVector3};

pub fn reflect(incident: UnitVector3<f64>, normal: UnitVector3<f64>) -> UnitVector3<f64> {
    let scale = 2.0 * incident.dot(&normal);
    // TODO: this can probably be unchecked
    Unit::new_normalize(incident.into_inner() - normal.scale(scale))
}
