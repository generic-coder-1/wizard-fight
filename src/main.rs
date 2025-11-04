use controller::Controller;
use iced::Font;

pub mod controller;
pub mod helper;
#[cfg(test)]
mod test;

fn main() -> iced::Result {
    iced::application("Wizard Fight", Controller::update, Controller::view)
        .font(include_bytes!("../assets/FiraCodeNerdFontMono-Regular.ttf"))
        .default_font(Font::with_name("FiraCode Nerd Font Mono"))
        .centered()
        .run()
}
