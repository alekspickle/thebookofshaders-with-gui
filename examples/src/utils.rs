use image::{self, ImageBuffer, ImageFormat, Rgb};
use std::path::Path;
use termion::{color, style};

#[derive(Debug)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Point2 {
    pub fn new(x: i32, y: i32) -> Self {
        Point2 { x, y }
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

// simplified full Bresenham line algorhitm implementation
//
// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
pub fn line(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p1: &Point2, p2: &Point2) {
    if (p2.y - p1.y) < (p2.x - p1.x).abs() {
        if p1.x > p2.x {
            plot_line_low(ib, p2, p1)
        } else {
            plot_line_low(ib, p1, p2)
        }
    } else {
        if p1.y > p2.y {
            plot_line_high(ib, p2, p1)
        } else {
            plot_line_high(ib, p1, p2)
        }
    }
}

pub fn plot_line_high(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p1: &Point2, p2: &Point2) {
    let mut dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut d = 2 * dx - dy;
    let mut x = p1.x;

    for y in p1.y..p2.y {
        assign_pixel(ib, x, y);

        if d > 0 {
            x = x + xi;
            d = d - 2 * dy;
        }
        d = d + 2 * dx
    }
}

pub fn plot_line_low(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p1: &Point2, p2: &Point2) {
    let dx = p2.x - p1.x;
    let mut dy = p2.y - p1.y;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = 2 * dy - dx;
    let mut y = p1.y;

    for x in p1.x..p2.x {
        assign_pixel(ib, x, y);

        if d > 0 {
            y = y + yi;
            d = d - 2 * dx;
        }
        d = d + 2 * dy
    }
}

pub fn assign_pixel(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: i32, y: i32) {
    // Mutating single pixel
    let pixel = ib.get_pixel_mut(x as u32, y as u32);
    // let data = (*pixel as Rgb<u8>).0;
    *pixel = Rgb([0.0 as u8, 0.0 as u8, 0.0 as u8]);
}
