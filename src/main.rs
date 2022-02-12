use clap::Parser;
use color::{Deg, ToRgb};
use colorgrad::Gradient;
use image::{Rgb, RgbImage};
use klask::Settings;
use rayon::prelude::*;
use std::{borrow::Cow, path::PathBuf};

const CX: f32 = -0.7;
const CY: f32 = 0.27015;

const MAX_ITER: i32 = 256;

#[derive(Debug, Parser, Clone, Default, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "800")]
    width: u32,

    #[clap(short, long, default_value = "600")]
    height: u32,

    #[clap(short, long, parse(from_os_str), default_value = "out.png")]
    output: PathBuf,

    #[clap(short, long, default_value = "1.0")]
    zoom: f32,

    #[clap(short, long, allow_hyphen_values = true, default_value = "0.0")]
    x_offset: f32,

    #[clap(short, long, allow_hyphen_values = true, default_value = "0.0")]
    y_offset: f32,

    #[clap(short, long, takes_value = false)]
    gui: bool,
}

fn main() {
    let mut settings = Settings::default();
    settings.custom_font = Some(Cow::Borrowed(include_bytes!(r"../Lato-Bold.ttf")));

    let args = Args::parse();
    if args.gui {
        klask::run_derived::<Args, _>(settings, |o| process(o));
    } else {
        process(args);
    }
}

fn process(args: Args) {
    /*
        if !opt.output.to_str().unwrap().trim().ends_with(".png") {
            eprintln!(
                "Only PNG supported, try --output {}.png",
                opt.output.display()
            );
            return;
        }
    */

    let (x_offset, y_offset) = (args.x_offset, args.y_offset);

    let w = args.width as f32;
    let h = args.height as f32;

    let mut img = RgbImage::new(args.width, args.height);

    img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let steps = convergence_steps(
                1.5 * (*x as f32 - w / 2.0) / (0.5 * args.zoom * w) + x_offset,
                1.0 * (*y as f32 - h / 2.0) / (0.5 * args.zoom * h) + y_offset,
            );

            //**pixel = colorgrade_1(steps);
            **pixel = colorgrad(steps, colorgrad::turbo());
            //dbg!(pixel);
        });

    img.save(args.output).unwrap();
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
