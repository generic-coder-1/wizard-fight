use super::{model::{position::Direction, spell::SpellElement}, view::controls::Control};

#[derive(Debug, Clone)]
pub enum Message {
    SpellSelect(SpellSelectMessage),
    Battle(BattleMessage),
}

#[derive(Debug, Clone)]
pub enum BattleMessage {
    TileSelect(usize, usize),
    ControlPageCycle(bool),
    SpellChoose(usize),
    DirectionSelect(Direction),
    ConfirmAction(Control),
}

#[derive(Debug, Clone)]
pub enum SpellSelectMessage {
    PointChange(PointChange),
    Confirm,
}

#[derive(Debug, Clone)]
pub struct PointChange {
    pub player: usize,
    pub increment: bool,
    pub element: SpellElement,
}
