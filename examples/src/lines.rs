use image::{ImageBuffer, Rgb};
use std::{path::Path, time::Instant};
use utils::Point2;

fn main() {
    let start = Instant::now();
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

    // Iterate over the coordinates and pixels of the image
    let horiz: Vec<i32> = imgbuf
        .enumerate_pixels_mut()
        .map(|(x, _, _)| x as i32)
        .step_by(step as usize)
        .collect();
    let mut vert: Vec<i32> = imgbuf
        .enumerate_pixels_mut()
        .map(|(_, y, _)| y as i32)
        .step_by(step as usize)
        .collect();

    // Dedup vector of y, because the image iterator takes pixel/per iteration
    vert.dedup_by(|a, b| a == b);
    let zipped = horiz.iter().zip(vert.iter());

    for (x, y) in zipped {
        // if x < &(imgbuf.width() as i32) && y < &(imgbuf.height() as i32) {
        // println!("x:{}, y:{}", x, y);

        draw(&mut imgbuf, *x, *y, step, step);
        // }
    }

    utils::save_image(imgbuf, Path::new("examples/outputs/lines.png"));
    println!("Time taken: {:?}", start.elapsed());
}

fn draw(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: i32, y: i32, w: i32, h: i32) {
    // left to right bool
    let left_to_right = rand::random::<f32>();
    if x + w < ib.width() as i32 && y + h < ib.height() as i32 {
        if left_to_right >= 0.5 {
            utils::line(ib, &Point2::new(x, y), &Point2::new(x + w, y + h));
        } else {
            utils::line(ib, &Point2::new(x + w, y), &Point2::new(x, y + h));
        }
    }
}
