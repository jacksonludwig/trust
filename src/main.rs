use iced::{Sandbox, Settings};

mod client;
mod gui;
mod handler;
mod host;

fn main() -> std::io::Result<()> {
    gui::App::run(Settings::default());

    Ok(())
}
