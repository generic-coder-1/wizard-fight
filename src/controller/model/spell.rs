use strum_macros::EnumIter;

use super::{position::Position, Battle};

#[derive(strum_macros::EnumIter, Clone, Copy, Debug)]
pub enum Spell {
    //water
    IncreasedCirculation = 0,
    WaterSpear,
    ManaDrain,
    Stagnation,

    //fire
    Flame,
    Fireball,
    Explosion,
    AuraOfFire,

    //earth
    StoneSkin,
    Spikes,
    Boulder,
    Wall,

    //wind
    WindBolt,
    Glide,
    RepulsiveBlast,
    Tornado,
}

impl Spell {
    pub fn requirement(self) -> (SpellElement, usize) {
        let points = self as usize % 4;
        let element = match self as usize / 4 {
            0 => SpellElement::Water,
            1 => SpellElement::Fire,
            2 => SpellElement::Earth,
            3 => SpellElement::Wind,
            _ => unreachable!(),
        };
        (element, points + 1)
    }

    pub fn spell_input_type(self) -> &'static SpellInputType {
        match self {
            Self::IncreasedCirculation
            | Self::ManaDrain
            | Self::AuraOfFire
            | Self::StoneSkin
            | Self::RepulsiveBlast
            | Self::Tornado => &SpellInputType::None,
            Self::WaterSpear
            | Self::Flame
            | Self::Fireball
            | Self::Boulder
            | Self::WindBolt
            | Self::Glide => &SpellInputType::Direction,
            Spell::Stagnation => &SpellInputType::Position(1),
            Spell::Explosion => &SpellInputType::Position(2),
            Spell::Spikes => &SpellInputType::Position(3),
            Spell::Wall => &SpellInputType::Position(0),
        }
    }
}

pub enum SpellInputType {
    None,
    Position(usize),
    Direction,
}

macro_rules! distance_spell_position_filter {
    ($max:expr) => {
        &|battle: &Battle, pos: Position| {
            let wiz = battle.get_current_wizard();
            let dist = wiz.position.apply(usize::abs_diff, pos);
            dist.x + dist.y < ($max)
        }
    };
}

pub const SPELL_POSITION_FILTER: &[&'static dyn Fn(&Battle, Position) -> bool] = &[
    distance_spell_position_filter!(5),
    &|battle: &Battle, pos: Position| {
        let wiz = battle.get_current_wizard();
        let dist = wiz.position.apply(usize::abs_diff, pos);
        battle.get_entity_at(pos).is_some_and(|ent| ent.is_wizard()) && (dist.x + dist.y < 8)
    },
    distance_spell_position_filter!(7),
    distance_spell_position_filter!(10),
];

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum SpellElement {
    Water,
    Fire,
    Earth,
    Wind,
}
