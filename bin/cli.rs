use clap::Parser;
use julia_set_renderer::{cli::Args, process};

fn main() {
    let args = Args::parse();
    process::<false>(args);
}
