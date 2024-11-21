use image::{ImageBuffer, Rgb};
use std::path::Path;
use utils::Timer;

fn main() {
    let _ = Timer::new();

    let mut imgbuf = ImageBuffer::new(500, 500);
    let step: i32 = 50;
    println!(
        "Juggling pixels ({}x{})...\nStep:{}.",
        imgbuf.width(),
        imgbuf.height(),
        step
    );

    // Iterate over the coordinates and pixels of the image
    // to set basic background color
    let scale = 0.55 + (step as f32) / (imgbuf.width() + imgbuf.height()) as f32;
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.1 * (x as f32).powf(scale) * (y as f32).powf(scale)) as u8;
        let g = (0.9 * (x as f32).powf(scale) * (y as f32).powf(scale)) as u8;
        let b = (0.7 * (x as f32).powf(scale) * (y as f32).powf(scale)) as u8;
        *pixel = Rgb([r, g, b]);
    }

    utils::save_image(imgbuf, Path::new("examples/outputs/basic.png"));
}
