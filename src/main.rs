use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bmp::{Image, Pixel, px};
use rand::Rng;

use crate::camera::Camera;
use crate::config::{CameraParams, Config, FilmParams};
use crate::film::Film;
use crate::ray::Hit;
use crate::shaders::Phong;
use crate::shaders::Shader;
use crate::util::Color3;
use std::time::Instant;

mod attributes;
mod camera;
mod config;
mod hittables;
mod lights;
mod materials;
mod ray;
mod shaders;
mod util;
mod film;
mod debug;

fn render_tile(mut tile: Film, camera: Camera, config: Arc<Config>) -> Film {
    let shader = Phong;
    let eye = config.camera.look_from.clone();

    let map_hit = |hit: Hit| {
        let color = shader.shade(
            &hit,
            eye,
            &config.lights,
            config.world.deref(),
            &config.march,
        );
        color.scale(255.99)
    };

    let FilmParams { width, height } = config.film;
    let aa = config.performance.anti_aliasing;
    let mut rng = rand::thread_rng();

    for (x, y) in tile.iter() {
        let mut color = Color3::default();
        for _ in 0..aa {
            let (ox, oy) = tile.offset(x, y);
            let (rx, ry) = (ox as f64 + rng.gen_range(0.0..1.0), oy as f64 + rng.gen_range(0.0..1.0));
            let ray = camera.ray(rx / width as f64, ry / height as f64);

            color += match ray.march(config.world.deref(), eye, &config.march) {
                None => continue,
                Some(surface) => map_hit(surface),
            };
        }
        let px = color.scale(1.0 / aa as f64);
        tile.set_pixel(x, y, px!(px.x, px.y, px.z));
    }

    tile
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let filename = match args.deref() {
        [_, filename] => Ok(filename),
        [] | [_] => Err("no config path given"),
        _ => Err("too many parameters, expected one"),
    }?;

    println!("Reading from: {filename}");

    let mut output = PathBuf::from(filename);
    output.set_extension("bmp");

    let path = Path::new("scenes").join(filename);
    let config = Arc::new(Config::parse(path)?);

    // println!("{:#?}", config.world);

    let CameraParams {
        look_from,
        look_at,
        up,
        fov,
    } = config.camera.clone();

    let FilmParams { width, height } = config.film;
    let aspect_ratio = width as f64 / height as f64;
    let camera = Camera::new(look_from, look_at, up, fov, aspect_ratio);

    let render = move |tile| {
        let camera = camera.clone();
        let config = config.clone();
        let f = move || render_tile(tile, camera, config);
        std::thread::spawn(f)
    };

    let start = Instant::now();

    let handles: Vec<_> = Film::rasterized(width, height).into_iter().map(render).collect();
    let films = handles.into_iter().map(|h| h.join().unwrap()).collect();

    println!("Time elapsed: {:.2?}", start.elapsed());

    let image = Film::combine(films);
    image.save(output)?;

    Ok(())
}
