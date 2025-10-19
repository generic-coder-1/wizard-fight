use super::{
    position::{Direction, Position},
    wizard::Team,
};

pub struct Projectile {
    pub position: Position,
    projectile_type: ProjectileType,
    damage: usize,
    direction: Direction,
    owner: Team,
    guiding: bool,
    speed: usize,
    lifetime: usize,
}

pub enum ProjectileType {
    Fireball,
    Spike,
    Boulder,
    Wall,
    WindBolt,
}
