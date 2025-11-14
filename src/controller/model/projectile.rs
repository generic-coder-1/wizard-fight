use super::{
    position::{Direction, Position},
    wizard::Team,
};

pub struct Projectile {
    pub position: Position,
    projectile_type: ProjectileType,
    pub damage: usize,
    direction: Direction,
    owner: Team,
    guiding: bool,
    speed: usize,
    pub passable: bool,
    lifetime: usize,
}

impl Projectile{
    pub fn take_damage(&mut self, damage: usize){
        self.damage = self.damage.saturating_sub(damage*2);
    }
}

pub enum ProjectileType {
    Fireball,
    Spike,
    Boulder,
    Wall,
    WindBolt,
}
