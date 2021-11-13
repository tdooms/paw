use std::ops::Deref;
use std::path::{Path, PathBuf};

use bmp::{px, Image, Pixel};

use crate::camera::Camera;
use crate::config::{parse_config, CameraParams, FilmParams};
use crate::ray::Hit;
use crate::shaders::Phong;
use crate::shaders::Shader;

mod attributes;
mod camera;
mod config;
mod hittables;
mod lights;
mod materials;
mod ray;
mod shaders;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let path = match args.deref() {
        [_, path] => Ok(path),
        [] | [_] => Err("no config path given"),
        _ => Err("too many parameters, expected one"),
    }?;

    let config = parse_config(Path::new("scenes").join(path))?;

    let CameraParams {
        look_from,
        look_at,
        up,
        fov,
    } = config.camera;

    let FilmParams { width, height } = config.film;
    let aspect_ratio = width as f64 / height as f64;

    let camera = Camera::new(look_from, look_at, up, fov, aspect_ratio);
    let eye = look_from;

    let mut film = Image::new(width, height);
    let shader = Phong;

    let map_hit = |hit: Hit| {
        let color = shader.shade(
            &hit,
            eye,
            &config.lights,
            config.world.deref(),
            &config.march,
        );
        let scaled = color.scale(255.99);
        px!(scaled.x, scaled.y, scaled.z)
    };

    for (x, y) in film.coordinates() {
        let ray = camera.ray(x as f64 / width as f64, y as f64 / height as f64);

        let pixel = match ray.march(config.world.deref(), eye, &config.march) {
            None => px!(0, 0, 0),
            Some(surface) => map_hit(surface),
        };

        film.set_pixel(x, y, pixel);
    }

    let mut output = PathBuf::from(path);
    output.set_extension("bmp");
    film.save(output)?;
    Ok(())
}
