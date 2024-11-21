//! [video](https://www.youtube.com/watch?v=m9joBL&OZVEo)
//!
//! Inspired by the labyrinth on Commodore64 [in this video](video)
//! It's not the same, but I like it better
//!

use image::{ImageBuffer, Rgb};
use rand::Rng;
use std::path::Path;
use utils::{Image, Timer};

const DIM: u32 = 500;

fn main() {
    let t = Timer::new();

    let mut image: Image = ImageBuffer::new(DIM, DIM).into();
    let buf = image.buf_mut();
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(5, DIM / 3);
    println!(
        "Building labyrinth ({}x{})...\nStep:{}.",
        buf.width(),
        buf.height(),
        step
    );

    // Set basic background color
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

    utils::save_image(image.into(), Path::new("examples/outputs/lines.png"));
    t.end();
}
