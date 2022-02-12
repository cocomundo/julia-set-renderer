use clap::Parser;
use color::{Deg, ToRgb};
use colorgrad::Gradient;
use image::{Rgb, RgbImage};
use rayon::prelude::*;
use std::path::PathBuf;

const CX: f32 = -0.7;
const CY: f32 = 0.27015;

const MAX_ITER: i32 = 256;
const ZOOM: f32 = 4.0;

#[derive(Debug, Parser, Clone, Default, PartialEq, Eq)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "800")]
    width: u32,

    #[clap(short, long, default_value = "600")]
    height: u32,

    #[clap(short, long, required = true, parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let opt = Args::parse();

    let (move_x, move_y) = (0.95, -0.15);

    let w = opt.width as f32;
    let h = opt.height as f32;

    let mut img = RgbImage::new(opt.width, opt.height);
    img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let steps = convergence_steps(
                1.5 * (*x as f32 - w / 2.0) / (0.5 * ZOOM * w) + move_x,
                1.0 * (*y as f32 - h / 2.0) / (0.5 * ZOOM * h) + move_y,
            );

            //**pixel = colorgrade_1(steps);
            **pixel = colorgrad(steps, colorgrad::turbo());
            //dbg!(pixel);
        });

    img.save(opt.output).unwrap();
}

fn convergence_steps(mut zx: f32, mut zy: f32) -> i32 {
    let mut i = MAX_ITER;
    while zx * zx + zy * zy < 4.0f32 && i > 1 {
        let tmp = zx * zx - zy * zy + CX;
        (zy, zx) = (2.0 * zx * zy + CY, tmp);
        i -= 1;
    }
    i
}

fn delphi_gradient(i: i32) -> Rgb<u8> {
    let r = (i >> 5) * 36;
    let g = ((i >> 3) & 7) * 36;
    let b = (i & 3) * 85;
    Rgb([r as u8, g as u8, b as u8])
}

fn smooth_gradient(i: i32) -> Rgb<u8> {
    let r = i * 5;
    let g = (i + 30) * 10;
    let b = i * 3;
    Rgb([r as u8, g as u8, b as u8])
}

fn sidef_gradient(i: i32) -> Rgb<u8> {
    let hsv = color::Hsv::<f32>::new(Deg(i as f32 / MAX_ITER as f32 * 360.0), 1.0, i as f32);
    let rgb = hsv.to_rgb::<u8>();
    Rgb([rgb.r, rgb.g, rgb.b])
}

fn colorgrad(i: i32, gradient: Gradient) -> Rgb<u8> {
    let scaled = 1.0f64 / MAX_ITER as f64 * i as f64;

    let val = gradient.at(scaled);
    Rgb([
        (val.r * 255.0) as u8,
        (val.g * 255.0) as u8,
        (val.b * 255.0) as u8,
    ])
}
