use board::Board;
use effects::Effects;
use projectile::Projectile;
use spell::Spell;
use strum::{EnumCount, IntoEnumIterator};
use wizard::{Team, Wizard};

mod board;
mod effects;
mod position;
mod projectile;
pub mod spell;
mod wizard;

pub enum Model {
    Battle(Battle),
    SpellSelect(SpellSelect),
}

impl Default for Model {
    fn default() -> Self {
        Self::SpellSelect(SpellSelect {
            players: vec![SpellChoice::default(); 2],
        })
    }
}

#[derive(Clone)]
pub struct SpellSelect {
    pub players: Vec<SpellChoice>,
}

#[derive(Clone)]
pub struct SpellChoice {
    pub water: usize,
    pub fire: usize,
    pub earth: usize,
    pub wind: usize,
    pub unused: usize,
}

impl Default for SpellChoice {
    fn default() -> Self {
        Self {
            water: 0,
            fire: 0,
            earth: 0,
            wind: 0,
            unused: 6,
        }
    }
}

impl From<&SpellChoice> for Vec<Spell> {
    fn from(value: &SpellChoice) -> Self {
        Spell::iter()
            .filter(|spell| {
                let (ele, point) = spell.requirement();
                point
                    <= match ele {
                        spell::SpellElement::Water => value.water,
                        spell::SpellElement::Fire => value.fire,
                        spell::SpellElement::Earth => value.earth,
                        spell::SpellElement::Wind => value.wind,
                    }
            })
            .collect()
    }
}

pub struct Battle {
    board: Board,
    wizards: Vec<Wizard>,
    current_player: usize,
    projectiles: Vec<Projectile>,
}

impl Battle {
    pub fn new(spell_select: &SpellSelect) -> Self {
        let wizards = spell_select
            .players
            .iter()
            .zip(Team::iter())
            .enumerate()
            .map(|(i, (spell_choice, team))| Wizard {
                team,
                health: 100,
                mana: 100,
                effects: [0; Effects::COUNT],
                position: (i, 0_usize).into(),
                spells: spell_choice.into(),
            })
            .collect();
        let projectiles = vec![];
        Self {
            board: Board::new(&wizards, &projectiles),
            wizards,
            projectiles,
            current_player: 0,
        }
    }
}
