use julia_set_renderer::{cli::Args, process};
use klask::Settings;
use std::borrow::Cow;

fn main() {
    let mut settings = Settings::default();
    settings.custom_font = Some(Cow::Borrowed(include_bytes!(r"../Lato-Bold.ttf")));

    klask::run_derived::<Args, _>(settings, process::<true>);
}
