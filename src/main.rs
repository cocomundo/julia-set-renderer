use std::path::PathBuf;

use image::{ImageBuffer, Pixel, Rgb};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
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

    for i in 0..opt.width / 2 {
        for j in 0..opt.height / 2 {
            let pixel = Rgb::from_channels(i as u8, j as u8, 60, 0);
            img.put_pixel(i as u32, j as u32, pixel);
        }
    }
    img.save(opt.output).unwrap();
}
