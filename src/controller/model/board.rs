use std::array::from_fn;

use super::{
    position::Position,
    projectile::Projectile,
    wizard::Wizard,
};

pub const WIDTH: usize = 30;
pub const HEIGHT: usize = 20;

pub struct Board {
    board: [Option<Entity>; WIDTH * HEIGHT],
}

impl Board {
    fn pos_to_index(pos: Position) -> usize {
        pos.x + pos.y * WIDTH
    }

    pub fn new(wizards: &[Wizard], projectiles: &[Projectile]) -> Self {
        let mut temp = Self::empty();
        wizards.iter().enumerate().for_each(|(i, wizard)| {
            temp.board[Board::pos_to_index(wizard.position)] = Some(Entity::Wizard(i))
        });
        projectiles.iter().enumerate().for_each(|(i, projectile)| {
            temp.board[Board::pos_to_index(projectile.position)] = Some(Entity::Projectile(i))
        });
        temp
    }

    pub fn empty() -> Self {
        Self {
            board: from_fn(|_| None),
        }
    }

    pub fn get_entity_at(&self, position: Position) -> Option<Entity> {
        self.board.get(Self::pos_to_index(position)).cloned()?
    }
}

#[derive(Clone, Copy)]
pub enum Entity {
    Wizard(usize),
    Projectile(usize),
}
