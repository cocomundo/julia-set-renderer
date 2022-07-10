use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser, Clone, Default, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value_t = 800)]
    pub width: u32,

    #[clap(short, long, default_value_t = 600)]
    pub height: u32,

    #[clap(short, long, parse(from_os_str), default_value = "out.png")]
    pub output: PathBuf,

    #[clap(short, long, default_value_t = 1.0)]
    pub zoom: f64,

    #[clap(short, long, allow_hyphen_values = true, default_value_t = 0.0)]
    pub x_offset: f64,

    #[clap(short, long, allow_hyphen_values = true, default_value_t = 0.0)]
    pub y_offset: f64,

    #[clap(long, takes_value = false)]
    pub open_after: bool,
}
