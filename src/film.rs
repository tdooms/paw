use bmp::{Image, Pixel};
use nalgebra::{vector, Vector2};

pub struct Film {
    buffer: Vec<Pixel>,

    dims: Vector2<u32>,
    start: Vector2<u32>,
}

impl Film {
    fn new(start: Vector2<u32>, dims: Vector2<u32>) -> Self {
        Self { start, dims, buffer: vec![Pixel::new(0, 0, 0); (dims.x * dims.y) as usize] }
    }

    pub fn rasterized(width: u32, height: u32) -> Vec<Self> {
        let iter_x = |s| (0..height).step_by(16).skip(s);
        let iter_y = |s| (0..width).step_by(16).skip(s);

        let capacity = (width / 16 + 1) * (height / 16 + 1);
        let mut result = Vec::with_capacity(capacity as usize);

        // TODO: also make smaller edge tiles in case step doesn't divide size
        for y in iter_y(0).zip(iter_y(1)) {
            for x in iter_x(0).zip(iter_x(1)) {
                let (start, end) = (vector![x.0, y.0], vector![x.1, y.1]);
                let tile = Film::new(start, end - start);
                result.push(tile)
            }
        }

        result
    }

    pub fn iter(&self) -> FilmIter {
        FilmIter { current: vector![0, 0], end: self.dims }
    }

    pub fn offset(&self, x: u32, y: u32) -> (u32, u32) {
        (x + self.start.x, y + self.start.y)
    }

    pub fn combine(films: Vec<Film>) -> Image {
        let end = films.iter().map(|f| f.start + f.dims).max_by_key(|e| e.x + e.y).unwrap();

        let mut image = Image::new(end.x, end.y);

        for film in films {
            for (x, y) in film.iter() {
                let (ox, oy) = film.offset(x, y);
                image.set_pixel(ox, oy, film.get_pixel(x, y))
            }
        }

        image
    }

    #[inline]
    pub fn set_pixel(&mut self, x: u32, y: u32, val: Pixel) {
        assert!(x < self.dims.x && y < self.dims.y);
        self.buffer[(y * self.dims.x + x) as usize] = val;
    }

    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        assert!(x < self.dims.x && y < self.dims.y);
        self.buffer[(y * self.dims.x + x) as usize]
    }
}

#[derive(Debug)]
pub struct FilmIter {
    current: Vector2<u32>,
    end: Vector2<u32>,
}

impl Iterator for FilmIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x >= self.end.x || self.current.y >= self.end.y {
            return None;
        }

        let result = (self.current.x, self.current.y);
        self.current.x += 1;

        if self.current.x == self.end.x {
            self.current.x = 0;
            self.current.y += 1;
        }

        Some(result)
    }
}