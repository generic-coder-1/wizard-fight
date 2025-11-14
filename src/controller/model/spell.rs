use strum_macros::{EnumIs, EnumIter};

use super::{
    position::{Direction, Position},
    Battle,
};

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
            Self::IncreasedCirculation | Self::StoneSkin => &SpellInputType::None(&|_, _| false),
            Self::AuraOfFire => &SpellInputType::None(&|battle, pos| {
                let wiz = battle.get_current_wizard();
                let dist = wiz.position.dist(pos);
                dist.x + dist.y <= 6
            }),
            Self::RepulsiveBlast => &SpellInputType::None(&|battle, pos| {
                let wiz = battle.get_current_wizard();
                let dist = wiz.position.dist(pos);
                (dist.x + dist.y <= 6)
                    && battle.get_entity_at(pos).is_some_and(|e| e.is_projectile())
            }),
            Self::Tornado => &SpellInputType::None(&|battle, pos| {
                let wiz = battle.get_current_wizard();
                let dist = wiz.position.dist(pos);
                dist.x + dist.y <= 5
            }),
            Self::Fireball | Self::Boulder | Self::WindBolt | Self::Glide => {
                &SpellInputType::Direction(&|battle, dir, pos| {
                    battle.get_current_wizard().position.move_in_direction(dir) == pos
                })
            }
            Spell::WaterSpear => &SpellInputType::Direction(&|battle, dir, pos| {
                let wiz = battle.get_current_wizard();

                let dist = wiz.position.dist(pos);
                let signed_dist = wiz.position.signed_dist(pos);
                (dist.x + dist.y <= 6)
                    && (dist.x * dist.y == 0)
                    && match dir {
                        Direction::Up => signed_dist.1 < 0,
                        Direction::Down => signed_dist.1 > 0,
                        Direction::Left => signed_dist.0 < 0,
                        Direction::Right => signed_dist.0 > 0,
                    }
            }),
            Spell::Flame => &SpellInputType::Direction(&|battle, dir, pos| {
                let wiz = battle.get_current_wizard();

                let dist = wiz.position.dist(pos);
                let signed_dist = wiz.position.signed_dist(pos);
                (dist.x + dist.y <= 7)
                    && (dist.x * dist.y == 0)
                    && match dir {
                        Direction::Up => signed_dist.1 < 0,
                        Direction::Down => signed_dist.1 > 0,
                        Direction::Left => signed_dist.0 < 0,
                        Direction::Right => signed_dist.0 > 0,
                    }
            }),

            Spell::Stagnation => &SpellInputType::Position(1, &|battle, p1, p2| p1 == p2),
            Spell::Explosion => {
                &SpellInputType::Position(2, &|battle, p1, p2| p1.dist(p2).mag() <= 3)
            }

            Spell::Spikes => {
                &SpellInputType::Position(3, &|battle, p1, p2| p1.dist(p2).reduce(usize::max) <= 3)
            }
            Spell::Wall => &SpellInputType::Position(0, &|battle, p1, p2| {
                let wiz = battle.get_current_wizard();
                let dist = wiz.position.dist(p1).reduce(usize::max);
                if wiz.position.dist(p2).reduce(usize::max) != dist {
                    return false;
                }
                let s = |center: Position, other: Position| {
                    let (x, y) = center.signed_dist(other);
                    match (x > y, -x > y) {
                        (true, true) => -7 * y + x,
                        (true, false) => x + y,
                        (false, true) => -5 * x - y,
                        (false, false) => 3 * y - x,
                    }
                };
                let s1 = s(wiz.position, p1);
                let s2 = s(wiz.position, p2);
                let max = 8 * dist as isize;
                (s1 - s2).rem_euclid(max).min((s2 - s1).rem_euclid(max)) <= 2
            }),
            Self::ManaDrain => &SpellInputType::Position(4, &|_, _, _| false),
        }
    }
}

#[derive(EnumIs)]
pub enum SpellInputType {
    None(&'static dyn Fn(&Battle, Position) -> bool),
    Position(usize, &'static dyn Fn(&Battle, Position, Position) -> bool),
    Direction(&'static dyn Fn(&Battle, Direction, Position) -> bool),
}

macro_rules! distance_spell_position_filter {
    ($max:expr) => {
        &|battle: &Battle, pos: Position| {
            let wiz = battle.get_current_wizard();
            let dist = wiz.position.dist(pos);
            dist.x + dist.y <= ($max)
        }
    };
}

pub const SPELL_POSITION_FILTER: &[&'static dyn Fn(&Battle, Position) -> bool] = &[
    distance_spell_position_filter!(5),
    &|battle: &Battle, pos: Position| {
        let wiz = battle.get_current_wizard();
        let dist = wiz.position.dist(pos);
        battle.get_entity_at(pos).is_some_and(|ent| ent.is_wizard())
            && (wiz.position != pos)
            && (dist.x + dist.y < 8)
    },
    distance_spell_position_filter!(7),
    distance_spell_position_filter!(10),
    &|battle: &Battle, pos: Position| {
        let wiz = battle.get_current_wizard();
        let dist = wiz.position.dist(pos);
        battle.get_entity_at(pos).is_some_and(|ent| ent.is_wizard())
            && (wiz.position != pos)
            && (dist.x + dist.y < 5)
    },
];

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum SpellElement {
    Water,
    Fire,
    Earth,
    Wind,
}
