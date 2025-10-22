use iced::Color;
use strum::EnumCount;
use strum_macros::{Display, EnumCount};

use crate::helper::from_rgb8;

use super::{effects::Effects, position::Position, spell::Spell};

pub struct Wizard {
    pub team: Team,
    pub health: usize,
    pub mana: usize,
    pub effects: [usize; Effects::COUNT],
    pub position: Position,
    pub spells: Vec<Spell>,
}

#[derive(strum_macros::EnumIter, EnumCount, Clone, Copy, Display)]
pub enum Team {
    Red,
    Blue,
}

pub const WIZARD_COLORS: [Color; Team::COUNT] = [
    from_rgb8(255, 0, 0),
    from_rgb8(0, 0, 255),
];
