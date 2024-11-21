use image::{ImageBuffer, Rgb};
use std::path::Path;

#[derive(Debug)]
struct Point2 {
    x: u32,
    y: u32,
}

impl Point2 {
    fn new(x: u32, y: u32) -> Self {
        Point2 { x, y }
    }
}

fn main() {
    let mut imgbuf = ImageBuffer::new(5, 5);
    let step: u32 = 80;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = Rgb([r, 0, b]);
    }

    for x in 0..imgbuf.width() {
        for y in 0..imgbuf.height() {
            imgbuf = draw( imgbuf.clone(), x, y, step, step);
        }
    }


    utils::save_image(imgbuf, Path::new("output/lines.png"));
}

fn draw(ib: ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32, w: u32, h: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
    // left to right bool
    let left_to_right = rand::random::<f32>();

    if left_to_right >= 0.5 {
        line(Point2::new(x, y), Point2::new(x + w, y + h));
    } else {
        line(Point2::new(x + w, y), Point2::new(x, y + h));
    }
}

fn line(p1: Point2, p2: Point2) {
    // let 

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
