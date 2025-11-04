pub mod message;
pub mod model;
pub mod view;

use iced::{
    widget::pane_grid::{self, Axis, Configuration},
    Task,
};
use message::{BattleMessage, Message, SpellSelectMessage};
use model::{
    position::Direction, spell::{SpellInputType, SPELL_POSITION_FILTER}, Battle, Model, SpellSelect
};

pub struct Controller {
    model: Model,
    quit: bool,
    fullscreen: bool,
    battle_panes: pane_grid::State<BattlePane>,
    hovered_tile: (usize, usize),
    control_page: isize,
    selected_tile: Option<(usize, usize)>,
    current_spell_index: Option<usize>,
    current_direction: Option<Direction>,
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
            hovered_tile: (0, 0),
            selected_tile: None,
            control_page: 0,
            current_spell_index: None,
            current_direction: None,
        }
    }
}

impl Controller {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SpellSelect(spell_select_message) => {
                self.update_spell_select(spell_select_message)
            }
            Message::Battle(battle_message) => self.update_battle_message(battle_message),
        };
        if self.quit {
            iced::exit()
        } else {
            Task::none()
        }
    }

    pub fn update_battle_message(&mut self, message: BattleMessage) {
        let Model::Battle(battle) = &mut self.model else {
            return;
        };
        match message {
            BattleMessage::TileSelect(x, y) => {
                        self.hovered_tile = (x, y);
                        if let Some(index) = self.current_spell_index {
                            if let SpellInputType::Position(filter_index) =
                                battle.get_current_wizard().spells[index].spell_input_type()
                            {
                                if (SPELL_POSITION_FILTER[*filter_index])(battle, (x, y).into()) {
                                    self.selected_tile = Some((x, y));
                                }
                            }
                        }
                    }
            BattleMessage::ControlPageCycle(forward) => {
                        self.control_page += if forward { 1 } else { -1 };
                    }
            BattleMessage::SpellChoose(spell_index) => {
                        self.current_spell_index = Some(spell_index);
                    }
            BattleMessage::DirectionSelect(direction) => self.current_direction = Some(direction)
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
