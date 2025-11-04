use board::{Board, Entity};
use effects::Effects;
use itertools::Itertools;
use position::Position;
use projectile::Projectile;
use spell::Spell;
use strum::{EnumCount, IntoEnumIterator};
use wizard::{Team, Wizard};

pub mod board;
pub mod effects;
pub mod position;
pub mod projectile;
pub mod spell;
pub mod wizard;

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
            .collect_vec();
        let projectiles = vec![];
        Self {
            board: Board::new(&wizards, &projectiles),
            wizards,
            projectiles,
            current_player: 0,
        }
    }

    pub fn get_entity_at(&self, position: Position) -> Option<Entity> {
        self.board.get_entity_at(position)
    }

    pub fn get_wizard(&self, entity: usize) -> &Wizard {
        &self.wizards[entity]
    }

    pub fn get_current_wizard(&self) -> &Wizard{
        &self.wizards[self.current_player]
    }

    pub fn get_projectile(&self, entity: usize) -> &Projectile {
        &self.projectiles[entity]
    }
}
