use image::{ImageBuffer, Rgb};
use std::path::Path;

#[derive(Debug)]
struct Point2 {
    x: i32,
    y: i32,
}

impl Point2 {
    fn new(x: i32, y: i32) -> Self {
        Point2 { x, y }
    }
}

fn main() {
    let mut imgbuf = ImageBuffer::new(50, 50);
    let step: i32 = 20;

    // Iterate over the coordinates and pixels of the image
    let pixel_coords: Vec<(i32, i32)> = imgbuf
        .enumerate_pixels_mut()
        .map(|(x, y, _)| (x as i32, y as i32))
        .collect();

    for (x, y) in pixel_coords.iter().step_by(step as usize) {
        if x < &(imgbuf.width() as i32) && y < &(imgbuf.height() as i32) {
            println!("x:{},y:{}", x, y);
            draw(&mut imgbuf, *x, *y, step, step);
        }
    }

    utils::save_image(imgbuf, Path::new("examples/outputs/lines.png"));
}

fn draw(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: i32, y: i32, w: i32, h: i32) {
    // left to right bool
    let left_to_right = rand::random::<f32>();

    if left_to_right >= 0.5 {
        line(ib, Point2::new(x, y), Point2::new(x + w, y + h));
    } else {
        line(ib, Point2::new(x + w, y), Point2::new(x, y + h));
    }
}

// simplified full Bresenham line algorhitm implementation
//
// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
fn line(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p1: Point2, p2: Point2) {
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

    // // Iterate over the coordinates and pixels of the image
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     let r = (0.3 * x as f32) as u8;
    //     let b = (0.3 * y as f32) as u8;
    //     *pixel = Rgb([r, 0, b]);
    // }

    // // Mutating single pixel
    // let pixel = imgbuf.get_pixel_mut(x, y);
    // let data = (*pixel as Rgb<u8>).0;
    // *pixel = Rgb([data[0], i as u8, data[2]]);
}

fn plot_line_high(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p1: Point2, p2: Point2) {
    let mut dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut D = 2 * dx - dy;
    let mut x = p1.x;

    for y in p1.y..p2.y {
        // Mutating single pixel
        let pixel = ib.get_pixel_mut(x as u32, y as u32);
        let data = (*pixel as Rgb<u8>).0;
        *pixel = Rgb([data[0], 0.9 as u8, data[2]]);

        if D > 0 {
            x = x + xi;
            D = D - 2 * dy;
        }
        D = D + 2 * dx
    }
}
fn plot_line_low(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p1: Point2, p2: Point2) {
    let dx = p2.x - p1.x;
    let mut dy = p2.y - p1.y;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut D = 2 * dy - dx;
    let mut y = p1.y;

    for x in p1.x..p2.x {
        // Mutating single pixel
        let pixel = ib.get_pixel_mut(x as u32, y as u32);
        let data = (*pixel as Rgb<u8>).0;
        *pixel = Rgb([data[0], 0.9 as u8, data[2]]);

        if D > 0 {
            y = y + yi;
            D = D - 2 * dx;
        }
        D = D + 2 * dy
    }
}
