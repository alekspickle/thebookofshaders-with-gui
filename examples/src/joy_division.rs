use image::{ImageBuffer, Rgb};
use itertools;
use itertools_num::linspace;
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use std::{path::Path, time::Instant};
use utils::{Line, Point2};

fn main() {
    let start = Instant::now();
    let mut imgbuf = ImageBuffer::new(50, 50);
    let step: f32 = 5.;
    let wiggle = 0.2;
    println!(
        "Juggling pixels ({}x{})...\nStep:{}.",
        imgbuf.width(),
        imgbuf.height(),
        step
    );

    // Iterate over the coordinates and pixels of the image
    // to set basic background color
    // @scale - make it kind of responsive to resize i guess
    let scale = 0.55 + (step as f32) / (imgbuf.width() + imgbuf.height()) as f32;
    println!("Scale:{}", scale);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.1 * (x as f32).powf(scale) * (y as f32).powf(scale)) as u8;
        let g = (0.9 * (x as f32).powf(scale) * (y as f32).powf(scale)) as u8;
        let b = (0.7 * (x as f32).powf(scale) * (y as f32).powf(scale)) as u8;
        *pixel = Rgb([r, g, b]);
    }

    // Iterate over the coordinates and pixels of the image
    // IMPORTANT STEP: it defines the shape of curves in the image
    let mut horiz: Vec<f32> = imgbuf
        .clone()
        .enumerate_pixels_mut()
        // .step_by(step as usize)
        // .filter(|(x, _, _)| *x as f32 % step == 0.0)
        .map(|(x, _, _)| x as f32)
        .take_while(|x| *x <= (imgbuf.width() as f32))
        .collect();
    let mut vert: Vec<f32> = imgbuf
        .clone()
        .enumerate_pixels_mut()
        .step_by(step as usize)
        .filter(|(_, y, _)| *y as f32 % step == 0.0)
        .map(|(_, y, _)| y as f32)
        .take_while(|y| *y <= (imgbuf.height() as f32))
        .collect();

    // Divide x coords by slices belonging to one y
    let horiz: Vec<&[f32]> = horiz.windows(imgbuf.width() as usize).collect();
    // Dedup vector of y, each y will be the y of a whole line
    vert.dedup_by(|a, b| a == b);

    println!("H:{:?}", horiz.len());
    print!("V:{:?}", vert);

    let mut lines: Vec<Line> = Vec::new();
    let rng = thread_rng();
    let range = Uniform::new_inclusive(0., wiggle);
    // for each Line
    for &i in vert.iter() {
        let i = i as f32;
        let mut line = Vec::new();
        // iterate through slices of x
        for &&line_window in horiz.get(i as usize).iter() {
            // iterate through slice
            for &j in line_window.iter() {
                // println!("x:{}; y:{}", j,i);

                // TODO: random points
                let mut r = range.sample_iter(rng).next().unwrap();
                // println!("{}", r);

                // array bounds corner case guard
                let new_y = if i + r < imgbuf.height() as f32 {
                    i + r
                } else {
                    i
                };

                line.push(Point2::new(j, new_y));
            }
        }
        println!("{} points+", line.len());
        lines.push(Line::new(line))
    }
    println!("\n{} lines", lines.len());

    for l in lines.iter() {
        let _p0 = &l.points[0];
        // println!("points len:{}", &l.points.len());
        for (i, p) in l.points.iter().enumerate() {
            // first element check
            let p_start = if i < 1 { 0 } else { i - 1 };

            // println!("p_start:{}", p_start);
            // println!(
            //     "overflowed:{} {}>{}",
            //     i > *&l.points[p_start].y as usize,
            //     i,
            //     *&l.points[p_start].y as usize
            // );

            utils::line(&mut imgbuf, &l.points[p_start], p);
            // utils::assign_pixel(&mut imgbuf, l.points[p_start].x, l.points[p_start].y);
            // utils::assign_pixel(&mut imgbuf, p.x, p.y);
        }
    }
    utils::save_image(imgbuf, Path::new("examples/outputs/joy_division.png"));
    println!("Time taken: {:?}", start.elapsed());
}

// fn draw(ib: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: f32, y: f32, w: f32, h: f32) {
//     // left to right bool
//     let left_to_right = rand::random::<f32>();
//     if x + w < ib.width() as f32 && y + h < ib.height() as f32 {
//         if left_to_right >= 0.5 {
//             utils::line(ib, &Point2::new(x, y), &Point2::new(x + w, y + h));
//         } else {
//             utils::line(ib, &Point2::new(x + w, y), &Point2::new(x, y + h));
//         }
//     }
// }
