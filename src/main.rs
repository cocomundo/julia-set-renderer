use image::{ImageBuffer, Pixel, Rgb};
use std::path::PathBuf;
use structopt::StructOpt;

const CX: f32 = -0.7;
const CY: f32 = 0.27015;

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

    let (move_x, move_y) = (0.0, 0.0);
    let max_iter = 255;
    let zoom = 1.0f32;

    let w = opt.width as f32;
    let h = opt.height as f32;

    for x in 0..opt.width {
        for y in 0..opt.height {
            let zx = 1.5 * (x as f32 - w / 2.0) / (0.5 * zoom * w) + move_x;
            let zy = 1.0 * (y as f32 - h / 2.0) / (0.5 * zoom * h) + move_y;

            let i = convergence_steps(max_iter, zx, zy);

            // convert byte to RGB (3 bytes), kinda magic to get nice colors
            let r = (i << 3) as u8;
            let g = (i << 5) as u8;
            let b = (i << 4) as u8;
            let pixel = Rgb::from_channels(r, g, b, 0);
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }

    img.save(opt.output).unwrap();
}

fn convergence_steps(max_iter: i32, mut zx: f32, mut zy: f32) -> i32 {
    let mut i = max_iter;
    while zx * zx + zy * zy < 4.0f32 && i > 1 {
        let tmp = zx * zx - zy * zy + CX;
        (zy, zx) = (2.0 * zx * zy + CY, tmp);
        i -= 1;
    }
    i
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
