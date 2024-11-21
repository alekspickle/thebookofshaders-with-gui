use image::{self, ImageBuffer, ImageFormat, Rgb};
use itertools_num::linspace;
use line_drawing::Point;
use std::{path::Path, time::Instant};
use termion::{color, style};

pub struct Timer(Instant);

impl Timer {
    pub fn new() -> Self {
        Self(Instant::now())
    }
    pub fn end(&self) {
        println!("Time taken: {:?}", self.0.elapsed());
    }
}

#[derive(Debug)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl Point2 {
    pub fn new(x: f32, y: f32) -> Self {
        Point2 { x, y }
    }
}

impl From<&Point2> for (f32, f32) {
    fn from(p: &Point2) -> (f32, f32) {
        (p.x, p.y)
    }
}

impl From<(f32, f32)> for Point2 {
    fn from(tuple: (f32, f32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub points: Vec<Point2>,
}

impl Line {
    pub fn new(points: Vec<Point2>) -> Self {
        Line { points }
    }
}

pub fn save_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>, p: &Path) {
    print_italic(&format!("saving as {:?}...", p));

    let mut _image_file = image::open(p);

    match img.save_with_format(p, ImageFormat::Png) {
        Ok(_) => print_green("success!"),
        Err(err) => println!("failed to save {:?}", err),
    }
}

pub fn print_green(s: &str) {
    println!(
        "{}{}{}",
        color::Fg(color::Green),
        s,
        color::Fg(color::Reset)
    )
}

pub fn print_italic(s: &str) {
    println!("{}{}{}", style::Italic, s, style::Reset);
}

pub struct Image(ImageBuffer<Rgb<u8>, Vec<u8>>);

impl From<ImageBuffer<Rgb<u8>, Vec<u8>>> for Image {
    fn from(buf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self {
        Self(buf)
    }
}

impl From<Image> for ImageBuffer<Rgb<u8>, Vec<u8>> {
    fn from(image: Image) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        image.0
    }
}

impl Image {
    /// Get reference to the inner image buffer
    pub fn buf(&self) -> &ImageBuffer<Rgb<u8>, Vec<u8>> {
        &self.0
    }

    /// Get inner image buffer as mutable
    pub fn buf_mut(&mut self) -> &mut ImageBuffer<Rgb<u8>, Vec<u8>> {
        &mut self.0
    }

    pub fn line(&mut self, p1: &Point2, p2: &Point2) {
        let (p1, p2) = (p1.into(), p2.into());
        let pix = line_drawing::XiaolinWu::<f32, i32>::new(p1, p2);
        for ((x, y), value) in pix {
            self.assign_pixel(x, y, value);
        }
    }

    pub fn assign_pixel(&mut self, x: i32, y: i32, intensity: f32) {
        let pixel = self.buf_mut().get_pixel_mut(x as u32, y as u32);
        // Reverse to make black default value
        // TODO: support different colors
        let value = (255.0 - intensity * 255.0) as u8;
        *pixel = Rgb([value; 3]);
    }

    // /// Xiaolin Wu`s naive implementation of line drawing with antialiasing
    // pub fn line_antialised(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p1: &Point2, p2: &Point2) {
    //     let steep = (p2.y - p1.x).abs() > (p2.x - p1.x).abs();

    // }
}
