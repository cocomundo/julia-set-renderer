use image::{ImageBuffer, Pixel, Rgb};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone, Default, PartialEq, Eq)]
pub struct Opt {
    #[structopt(short, long, default_value = "800")]
    width: u32,

    #[structopt(short, long, default_value = "600")]
    height: u32,

    #[structopt(short, long, required = true, parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let mut img = ImageBuffer::new(opt.width, opt.height);

    for i in 0..opt.width {
        for j in 0..opt.height {
            img.put_pixel(i as u32, j as u32, delphi_gradient(i / 10));
        }
    }
    img.save(opt.output).unwrap();
}

fn delphi_gradient(i: u32) -> Rgb<u8> {
    let r = (i >> 5) * 36;
    let g = ((i >> 3) & 7) * 36;
    let b = (i & 3) * 85;
    Rgb::from_channels(r as u8, g as u8, b as u8, 0)
}

fn smooth_gradient(i: u32) -> Rgb<u8> {
    let r = i * 5;
    let g = (i + 30) * 10;
    let b = i * 3;
    Rgb::from_channels(r as u8, g as u8, b as u8, 0)
}
