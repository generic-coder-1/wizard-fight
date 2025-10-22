use controller::Controller;

pub mod controller;
pub mod helper;

fn main() -> iced::Result {
    iced::application("Wizard Fight", Controller::update, Controller::view)
        .centered()
        .run()
}
