use std::fs::{read_to_string, File};
use std::path::Path;
use std::rc::Rc;

use nalgebra::{point, Point3, UnitVector3, Vector, Vector3};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::hittables::Hittable;
use crate::lights::Light;

// When casting a ray, do not start at 0 to avoid colliding with the object itself
fn start_eps() -> f64 {
    0.001
}

// When the algorithm is near enough to an object, count as a hit
fn hit_eps() -> f64 {
    0.000001
}

// When sampling a normal use this as step
fn normal_eps() -> f64 {
    0.001
}

// End of the world, stop marching when outside this range
fn max_t() -> f64 {
    100.0
}

// Max steps a ray will march before deciding it hasn't hit anything
fn max_steps() -> u64 {
    1000
}

// Width of the output image
fn width() -> u32 {
    512
}

// Height of the output image
fn height() -> u32 {
    512
}

fn look_from() -> Point3<f64> {
    point![1.0, 1.0, -1.0]
}

fn look_at() -> Point3<f64> {
    point![0.0, 0.0, 0.0]
}

fn up() -> UnitVector3<f64> {
    Vector3::y_axis()
}

fn fov() -> f64 {
    std::f64::consts::FRAC_PI_2
}

trait HittableOwned = Hittable + DeserializeOwned;
trait LightOwned = Light + DeserializeOwned;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Settings {
    #[serde(default = "start_eps")]
    pub start_eps: f64,

    #[serde(default = "hit_eps")]
    pub hit_eps: f64,

    #[serde(default = "normal_eps")]
    pub normal_eps: f64,

    #[serde(default = "max_t")]
    pub max_t: f64,

    #[serde(default = "max_steps")]
    pub max_steps: u64,
}

// nalgebra vectors do not allow for const...
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct CameraParams {
    #[serde(default = "look_from")]
    pub look_from: Point3<f64>,

    #[serde(default = "look_at")]
    pub look_at: Point3<f64>,

    #[serde(default = "up")]
    pub up: UnitVector3<f64>,

    #[serde(default = "fov")]
    pub fov: f64,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct FilmParams {
    #[serde(default = "width")]
    pub width: u32,

    #[serde(default = "height")]
    pub height: u32,
}

#[derive(Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub camera: CameraParams,
    pub film: FilmParams,

    #[serde(default = "crate::scene::create_world")]
    pub world: Box<dyn HittableOwned>,

    #[serde(default = "crate::scene::create_lights")]
    pub lights: Vec<Box<dyn LightOwned>>,
}

pub fn parse_config(path: impl AsRef<Path>) -> Result<Config, Box<dyn std::error::Error>> {
    let result = serde_json::from_str(&read_to_string(path)?)?;
    Ok(result)
}
