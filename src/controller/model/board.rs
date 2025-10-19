use std::array::from_fn;

use super::{
    position::Position,
    projectile::Projectile,
    wizard::{self, Wizard},
};

pub const WIDTH: usize = 40;
pub const HEIGHT: usize = 20;

pub struct Board {
    board: [Option<Entity>; WIDTH * HEIGHT],
}

impl Board {
    fn pos_to_index(pos: Position) -> usize {
        pos.x + pos.y * WIDTH
    }

    pub fn new(wizards: &Vec<Wizard>, projectiles: &Vec<Projectile>) -> Self {
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
}

pub enum Entity {
    Wizard(usize),
    Projectile(usize),
}
