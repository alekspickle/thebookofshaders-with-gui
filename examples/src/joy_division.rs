use image::{ImageBuffer, Rgb};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng, Rng,
};
use std::path::Path;
use utils::{Image, Line, Point2, Timer};

const DIM: u32 = 500;
const NARROW: f32 = 0.3;

fn main() {
    let t = Timer::new();

    let mut image: Image = ImageBuffer::new(DIM, DIM).into();
    let buf = image.buf_mut();
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(5.0, DIM as f32 / 10.0);
    let wiggle = 0.2;
    println!(
        "Bringing joy ({}x{})...\nStep:{}.",
        buf.width(),
        buf.height(),
        step
    );

    // Iterate over the coordinates and pixels of the image
    // to set basic background color
    // @scale - make it kind of responsive to resize i guess
    // let scale = 0.55 + (step as f32) / (buf.width() + buf.height()) as f32;
    // println!("Scale:{}", scale);
    // for (x, y, pixel) in buf.enumerate_pixels_mut() {
    //     let r = (0.1 * (x as f32).powf(scale) * (y as f32).powf(scale)) as u8;
    //     let g = (0.9 * (x as f32).powf(scale) * (y as f32).powf(scale)) as u8;
    //     let b = (0.7 * (x as f32).powf(scale) * (y as f32).powf(scale)) as u8;
    //     *pixel = Rgb([r, g, b]);
    // }
    for (.., pixel) in buf.enumerate_pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    let (w, h) = (buf.width(), buf.height());
    // TODO: maybe it's possible to do nicer with these?..
    // let xs = (0..w).map(|u| u as f32).step_by(step as usize);
    // let ys = (0..h).map(|u| u as f32).step_by(step as usize);

    let pxs: Vec<(u32, u32)> = buf
        .enumerate_pixels()
        .filter(|p| p.0 % step == 0)
        .filter(|p| p.1 % step == 0)
        .map(|p| (p.0 as u32, p.1 as u32))
        .filter(|p| p.0 < w && p.1 < h)
        .collect();

    let tuples: Vec<((f32, f32), (f32, f32))> = pxs.into_iter().fold(vec![], |acc, el| {
        let (x, y) = (el.0 as f32, el.1 as f32);
        let p2 = {
            let step = step as f32;
            let x = if x > step && rand::random::<bool>() {
                x - step
            } else {
                x + step
            };
            let y = if y > step && rand::random::<bool>() {
                y - step
            } else {
                y + step
            };
            (x, y)
        };
        let mut new = acc;
        if p2.0 < w as f32 && p2.1 < h as f32 {
            new.push(((x, y), p2));
        }
        new
    });

    for (p1, p2) in tuples {
        image.line(&p1.into(), &p2.into());
    }

    // // Iterate over the coordinates and pixels of the image
    // // IMPORTANT STEP: it defines the shape of curves in the image
    // let horiz: Vec<f32> = buf
    //     .clone()
    //     .enumerate_pixels_mut()
    //     // .step_by(step as usize)
    //     // .filter(|(x, _, _)| *x as f32 % step == 0.0)
    //     .map(|(x, _, _)| x as f32)
    //     .take_while(|x| *x <= (buf.width() as f32))
    //     .collect();
    // let mut vert: Vec<f32> = buf
    //     .clone()
    //     .enumerate_pixels_mut()
    //     .step_by(step as usize)
    //     .filter(|(_, y, _)| *y as f32 % step == 0.0)
    //     .map(|(_, y, _)| y as f32)
    //     .take_while(|y| *y <= (buf.height() as f32))
    //     .collect();

    // // Divide x coords by slices belonging to one y
    // let horiz: Vec<&[f32]> = horiz.windows(buf.width() as usize).collect();
    // // Dedup vector of y, each y will be the y of a whole line
    // vert.dedup_by(|a, b| a == b);

    // let mut lines: Vec<Line> = Vec::new();
    // let rng = thread_rng();
    // let range = Uniform::new_inclusive(0., wiggle);
    // // for each Line
    // for &i in vert.iter() {
    //     let i = i as f32;
    //     let mut line = Vec::new();
    //     // iterate through slices of x
    //     for &&line_window in horiz.get(i as usize).iter() {
    //         // iterate through slice
    //         for &j in line_window.iter() {
    //             // println!("x:{}; y:{}", j,i);

    //             let mut r = range.sample_iter(rng).next().unwrap();
    //             // println!("{}", r);

    //             // array bounds corner case guard
    //             let new_y = if i + r < buf.height() as f32 {
    //                 i + r
    //             } else {
    //                 i
    //             };

    //             line.push(Point2::new(j, new_y));
    //         }
    //     }
    //     println!("{} points+", line.len());
    //     lines.push(Line::new(line))
    // }
    // println!("\n{} lines", lines.len());

    // for l in lines.iter() {
    //     let _p0 = &l.points[0];
    //     // println!("points len:{}", &l.points.len());
    //     for (i, p) in l.points.iter().enumerate() {
    //         // first element check
    //         let p_start = if i < 1 { 0 } else { i - 1 };

    //         // println!("p_start:{}", p_start);
    //         // println!(
    //         //     "overflowed:{} {}>{}",
    //         //     i > *&l.points[p_start].y as usize,
    //         //     i,
    //         //     *&l.points[p_start].y as usize
    //         // );

    //         image.line(&l.points[p_start], p);
    //         // utils::assign_pixel(&mut buf, l.points[p_start].x, l.points[p_start].y);
    //         // utils::assign_pixel(&mut buf, p.x, p.y);
    //     }
    // }

    utils::save_image(
        buf.to_owned(),
        Path::new("examples/outputs/joy_division.png"),
    );
    t.end();
}
