#[derive(strum_macros::EnumIter, Clone, Copy)]
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
        let element = match self as usize / 4{
            0 => SpellElement::Water,
            1 => SpellElement::Fire,
            2 => SpellElement::Earth,
            3 => SpellElement::Wind,
            _ => unreachable!()
        };
        (element, points)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpellElement {
    Water,
    Fire,
    Earth,
    Wind,
}
