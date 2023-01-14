use julia_set_renderer::{cli::Args, process};
use klask::Settings;

fn main() {
    let settings = Settings {
        custom_font: Some(include_bytes!(r"../Lato-Bold.ttf")),
        ..Default::default()
    };

    klask::run_derived::<Args, _>(settings, process::<true>);
}
