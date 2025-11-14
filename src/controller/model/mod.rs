use core::panic;
use std::collections::HashSet;

use board::{Board, Entity};
use effects::Effects;
use itertools::Itertools;
use position::{Direction, Position};
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
    valid_move_positions: HashSet<Position>,
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
        let mut temp = Self {
            board: Board::new(&wizards, &projectiles),
            wizards,
            projectiles,
            current_player: 0,
            valid_move_positions: HashSet::new(),
        };
        temp.update_valid_move_positons();
        temp
    }

    pub fn move_wizard_to(&mut self, wiz_i: usize, pos: Position) {
        let wiz = &mut self.wizards[wiz_i];
        match self.board.get_entity_at(pos) {
            None => {}
            Some(Entity::Projectile(p)) => {
                let proj = &self.projectiles[p];
                if !proj.passable {
                    panic!("trying to move into an impassible projectile");
                }
                wiz.take_damage(proj.damage);
            }
            Some(Entity::Wizard(_)) => panic!("trying to move a wizard into another wizard"),
        }
        self.board.remove_entity_at(pos);
        self.board.swap_enttities(wiz.position, pos);
        wiz.position = pos;
        self.update_valid_move_positons();
    }

    pub fn move_current_wizard_to(&mut self, pos: Position) {
        self.move_wizard_to(self.current_player, pos);
    }

    pub fn get_entity_at(&self, position: Position) -> Option<Entity> {
        self.board.get_entity_at(position)
    }

    pub fn get_wizard(&self, entity: usize) -> &Wizard {
        &self.wizards[entity]
    }

    pub fn get_current_wizard(&self) -> &Wizard {
        &self.wizards[self.current_player]
    }

    pub fn get_projectile(&self, entity: usize) -> &Projectile {
        &self.projectiles[entity]
    }

    pub fn wizard_can_move(&self, tile: Position) -> bool {
        self.valid_move_positions.contains(&tile)
    }

    fn can_move_from(&mut self, start: Position, depth: usize) {
        if self.get_entity_at(start).is_some_and(|e| -> bool {
            match e {
                Entity::Wizard(w) => self.current_player != w,
                Entity::Projectile(p) => !self.get_projectile(p).passable,
            }
        }) {
            return;
        }
        self.valid_move_positions.insert(start);
        if depth > 0 {
            Direction::iter().for_each(|dir| {
                let end = start.move_in_direction(dir);
                if !self.valid_move_positions.contains(&end) {
                    self.can_move_from(end, depth - 1);
                }
            });
        }
    }

    fn update_valid_move_positons(&mut self) {
        self.valid_move_positions.drain();

        let wiz = self.get_current_wizard();
        let mut max_distance = 2;
        //handle effects
        if wiz.has_effect(Effects::Circulation) {
            max_distance *= 2
        };
        if wiz.has_effect(Effects::Tornado) {
            max_distance *= 2
        };
        if wiz.has_effect(Effects::AuraOfFire) {
            max_distance /= 2
        };

        self.can_move_from(wiz.position, max_distance);
    }
}
