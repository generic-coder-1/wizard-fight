pub mod message;
pub mod model;
pub mod view;

use iced::widget::pane_grid::{self, Axis, Configuration};
use message::{Message, SpellSelectMessage};
use model::{Battle, Model, SpellSelect};

pub struct Controller {
    model: Model,
    quit: bool,
    fullscreen: bool,
    battle_panes: pane_grid::State<BattlePane>,
}

enum BattlePane {
    Battle,
    Info,
    Control,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            model: Model::default(),
            quit: false,
            fullscreen: false,
            battle_panes: pane_grid::State::with_configuration(Configuration::Split {
                axis: Axis::Vertical,
                ratio: 3.0 / 4.0,
                a: Box::new(Configuration::Pane(BattlePane::Battle)),
                b: Box::new(Configuration::Split {
                    axis: Axis::Horizontal,
                    ratio: 1.0 / 3.0,
                    a: Box::new(Configuration::Pane(BattlePane::Info)),
                    b: Box::new(Configuration::Pane(BattlePane::Control)),
                }),
            }),
        }
    }
}

impl Controller {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SpellSelect(spell_select_message) => {
                if let Model::SpellSelect(spell_select) = &mut self.model {
                    self.update_spell_select(spell_select_message)
                }
            }
        }
    }

    pub fn update_spell_select(&mut self, message: SpellSelectMessage) {
        let mut confirm = None;
        if let Model::SpellSelect(spell_select) = &mut self.model {
            match message {
                SpellSelectMessage::PointChange(message) => {
                    let player = &mut spell_select.players[message.player];
                    let num = match message.element {
                        model::spell::SpellElement::Water => &mut player.water,
                        model::spell::SpellElement::Fire => &mut player.fire,
                        model::spell::SpellElement::Earth => &mut player.earth,
                        model::spell::SpellElement::Wind => &mut player.wind,
                    };
                    if message.increment && player.unused > 0 && *num < 4 {
                        *num += 1;
                        player.unused -= 1;
                    }
                    if !message.increment && *num > 0 {
                        *num -= 1;
                        player.unused += 1;
                    }
                }
                SpellSelectMessage::Confirm => {
                    confirm = Some(spell_select.clone());
                }
            }
        }
        if let Some(spell_select) = confirm {
            self.model = Model::Battle(Battle::new(&spell_select));
        }
    }
}
