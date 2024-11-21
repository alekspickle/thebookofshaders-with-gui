use image::{self, ImageBuffer, ImageFormat, Rgb};
use std::path::Path;
use termion::{color, style};

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
