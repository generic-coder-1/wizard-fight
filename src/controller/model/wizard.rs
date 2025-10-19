use strum::EnumCount;

use super::{effects::Effects, position::Position, spell::Spell};

pub struct Wizard {
    pub team: Team,
    pub health: usize,
    pub mana: usize,
    pub effects: [usize; Effects::COUNT],
    pub position: Position,
    pub spells: Vec<Spell>,
}

#[derive(strum_macros::EnumIter)]
pub enum Team {
    Red,
    Blue,
}
