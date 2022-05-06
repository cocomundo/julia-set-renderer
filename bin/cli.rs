use clap::Parser;
use julia_set_renderer::process;
use julia_set_renderer::Args;

fn main() {
    let args = Args::parse();
    process::<false>(args);
}
