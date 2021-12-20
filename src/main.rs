use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short, long, default_value = "800")]
    width: usize,

    #[structopt(short, long, default_value = "600")]
    height: usize,

    #[structopt(short, long, required = true, parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("width: {}", opt.width);
    println!("height: {}", opt.height);
    println!("output path: {}", opt.output.display());
}
