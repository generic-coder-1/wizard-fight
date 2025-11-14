use iced::Color;
use strum::EnumCount;
use strum_macros::{Display, EnumCount};

use crate::helper::from_rgb8;

use super::{
    effects::{self, Effects},
    position::Position,
    spell::Spell,
};

pub struct Wizard {
    pub team: Team,
    pub health: usize,
    pub mana: usize,
    pub effects: [usize; Effects::COUNT],
    pub position: Position,
    pub spells: Vec<Spell>,
}

impl Wizard {
    pub fn has_effect(&self, effect: Effects) -> bool {
        self.effects[effect as usize] > 0
    }

    pub fn decrement_effects(&mut self) {
        self.effects
            .iter_mut()
            .for_each(|time| *time = time.saturating_sub(1));
    }

    pub fn take_damage(&mut self, damage: usize) {
        let mut damage_taken = damage;
        if self.has_effect(Effects::StoneSkin) {
            damage_taken /= 2
        };
        self.health = self.health.saturating_sub(damage_taken);
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }
}

#[derive(strum_macros::EnumIter, EnumCount, Clone, Copy, Display)]
pub enum Team {
    Red,
    Blue,
}

pub const WIZARD_COLORS: [Color; Team::COUNT] = [from_rgb8(255, 0, 0), from_rgb8(0, 0, 255)];
