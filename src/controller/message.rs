use super::model::spell::SpellElement;

#[derive(Debug, Clone)]
pub enum Message{
    SpellSelect(SpellSelectMessage),
    Battle(BattleMessage),
}

#[derive(Debug, Clone)]
pub enum BattleMessage{}

#[derive(Debug, Clone)]
pub enum SpellSelectMessage{
    PointChange(PointChange),
    Confirm,
}

#[derive(Debug, Clone)]
pub struct PointChange{
    pub player: usize,
    pub increment: bool,
    pub element: SpellElement
}
