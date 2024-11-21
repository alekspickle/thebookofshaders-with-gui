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
    let step = rng.gen_range(5, DIM / 10) as f32;
    let wiggle = (DIM / 50) as f32;
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
        *pixel = Rgb([255, 255, 240]);
    }

    let (w, h) = (buf.width() as f32, buf.height() as f32);
    let left = NARROW * w;
    let right = w - NARROW * w;
    let pxs: Vec<(f32, f32)> = buf
        .enumerate_pixels()
        .filter(|p| p.0 > left as u32 && p.0 < right as u32)
        .filter(|p| p.1 % (DIM / 30) == 0)
        .map(|p| (p.0 as f32, p.1 as f32))
        .filter(|p| p.0 < w && p.1 < h)
        .collect();

    let tuples: Vec<((f32, f32), (f32, f32))> = pxs.into_iter().fold(vec![], |acc, el| {
        let (x, y) = (el.0 as f32, el.1 as f32);
        let add = |t: ((f32, f32), (f32, f32))| {
            let mut new = acc.clone();
            if t.1 .0 < w && t.1 .1 < h {
                new.push(t);
            }
            new
        };

        match x {
            x if x < left => {
                let horiz = ((0.0f32, y), (left, y));
                if acc.contains(&horiz) {
                    acc
                } else {
                    add(horiz)
                }
            }
            x if x > right => {
                let horiz = ((right, y), (w - 1.0, y));
                if acc.contains(&horiz) {
                    acc
                } else {
                    add(horiz)
                }
            }
            x if x % wiggle == 0.0 => {
                let p2 = {
                    // tuples.extend(vec![((0.0, left), (right, w - 1.0))]);

                    let y = if x > left || x < right { y + wiggle } else { y };
                    let x = if rand::random::<bool>() {
                        if x + wiggle > DIM as f32 {
                            x
                        } else {
                            x + wiggle
                        }
                    } else {
                        if x - wiggle < 0.0 {
                            x
                        } else {
                            x - wiggle
                        }
                    };
                    println!("R:{x},{y}");

                    (x, y)
                };
                add(((x, y), p2))
            }
            _ => acc,
        }
    });

    // println!("tuples: {:?}", tuples);

    for (p1, p2) in tuples {
        image.line(&p1.into(), &p2.into());
    }

    utils::save_image(image.into(), Path::new("examples/outputs/joy_division.png"));
    t.end();
}
