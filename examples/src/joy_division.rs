use image::{ImageBuffer, Rgb};
use std::{path::Path, time::Instant};
use utils::{Line, Point2};

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
        .clone()
        .enumerate_pixels_mut()
        .map(|(x, _, _)| x as i32)
        .take_while(|x| *x < imgbuf.width() as i32)
        .step_by(step as usize)
        .collect();
    let mut vert: Vec<i32> = imgbuf
        .clone()
        .enumerate_pixels_mut()
        .map(|(_, y, _)| y as i32)
        .take_while(|y| *y < imgbuf.height() as i32)
        .step_by(step as usize)
        .collect();

    // Dedup vector of y, because the image iterator takes pixel/per iteration
    vert.dedup_by(|a, b| a == b);
    // let mut zipped: Vec<(i32, i32)> = horiz
    //     .into_iter()
    //     .zip(vert.into_iter())
    //     .take_while(|(x, y)| *x < imgbuf.width() as i32 && *y < imgbuf.height() as i32)
    //     .collect();

    let mut lines: Vec<Line> = Vec::new();
    for x in horiz.iter() {
        let mut line = Vec::new();
        for y in vert.iter() {
            line.push(Point2::new(*x, *y));
        }
        lines.push(Line::new(line))
    }

    for l in lines.iter() {
        let p0 = &l.points[0];
        for (i, p) in l.points.iter().enumerate() {
            let p_start = if i < 1 { 0 } else { i - 1 };
            utils::line(&mut imgbuf, &l.points[p_start], p)
        }
    }
    utils::save_image(imgbuf, Path::new("examples/outputs/joy_division.png"));
    println!("Time taken: {:?}", start.elapsed());
}

// fn draw(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: i32, y: i32, w: i32, h: i32) {
//     // left to right bool
//     let left_to_right = rand::random::<f32>();
//     if x + w < ib.width() as i32 && y + h < ib.height() as i32 {
//         if left_to_right >= 0.5 {
//             utils::line(ib, &Point2::new(x, y), &Point2::new(x + w, y + h));
//         } else {
//             utils::line(ib, &Point2::new(x + w, y), &Point2::new(x, y + h));
//         }
//     }
// }
