use controller::Controller;

pub mod controller;
pub mod helper;
#[cfg(test)]
mod test;

fn main() -> iced::Result {
    iced::application("Wizard Fight", Controller::update, Controller::view)
        .centered()
        .run()
}
