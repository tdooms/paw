use bmp::{px, Image, Pixel};
use nalgebra::{Point3, Vector3};

use crate::camera::Camera;
use crate::config::{parse_config, CameraParams, FilmParams};
use crate::ray::Hit;
use crate::shaders::Phong;
use crate::shaders::Shader;

mod camera;
mod config;
mod hittables;
mod lights;
mod materials;
mod ray;
mod scene;
mod shaders;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = parse_config("config.json")?;

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

    println!("{:#?}", config.world);

    for (x, y) in film.coordinates() {
        let ray = camera.ray(x as f64 / width as f64, y as f64 / height as f64);

        let map_hit = |hit: Hit| {
            let color = shader.shade(&hit, eye, &config.lights, &config.world);
            let scaled = color.scale(255.99);
            px!(scaled.x, scaled.y, scaled.z)
        };

        let pixel = match ray.march(&config.world, eye, &config.settings) {
            None => px!(0, 0, 0),
            Some(surface) => map_hit(surface),
        };

        film.set_pixel(x, y, pixel);
    }

    film.save("img.bmp")?;
    Ok(())
}
