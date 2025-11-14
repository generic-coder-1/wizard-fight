use controls::Control;
use iced::{
    alignment::Horizontal,
    widget::{
        button, column, container, pane_grid, responsive, row, stack, text::LineHeight, Button,
        Column, Container, PaneGrid, Row, Space, Text,
    },
    Alignment, Background, Border, Color, Element,
    Length::{self, Fill, FillPortion},
    Theme,
};
use itertools::Itertools;
use strum::{EnumCount, IntoEnumIterator};

use crate::{
    controller::{message::Message, Controller},
    helper::from_rgb8,
};

use super::{
    message::{BattleMessage, PointChange, SpellSelectMessage},
    model::{
        board,
        effects::Effects,
        spell::{SpellElement, SpellInputType, SPELL_POSITION_FILTER},
        wizard::WIZARD_COLORS,
        Battle, Model, SpellSelect,
    },
};

pub mod controls;

const GREY: Color = from_rgb8(50, 50, 50);
const YELLOW: Color = from_rgb8(200, 200, 0);
const GREEN: Color = from_rgb8(0, 200, 0);
const RED: Color = from_rgb8(200, 0, 0);
const ORANGE: Color = from_rgb8(255, 140, 0);

impl Controller {
    pub fn view(&self) -> Element<Message> {
        match &self.model {
            Model::Battle(battle) => self.view_battle(battle).map(Message::Battle),
            Model::SpellSelect(spell_select) => {
                Controller::view_spell_select(spell_select).map(Message::SpellSelect)
            }
        }
    }

    fn view_board<'a>(&'a self, battle: &'a Battle) -> Element<'a, BattleMessage> {
        let cell = move |x, y| {
            let value = battle.get_entity_at((x, y).into());
            let color = match value {
                Some(ent) => match ent {
                    board::Entity::Wizard(w) => WIZARD_COLORS[battle.get_wizard(w).team as usize],
                    board::Entity::Projectile(_p) => from_rgb8(32, 102, 219), //temporary
                },
                None => GREY,
            };
            button("")
                .style(move |_, _| button::Style {
                    background: Some(Background::Color(color)),
                    border: {
                        let mut b = Border::default().rounded(2.0);
                        match (x, y) {
                            val if val == self.hovered_tile => b = b.color(YELLOW).width(2.0),
                            val if self.selected_tile.is_some_and(|tile| val == tile)
                                && (self.current_spell_index.is_some_and(|i| {
                                    battle.get_current_wizard().spells[i]
                                        .spell_input_type()
                                        .is_position()
                                }) || self.get_control() == Control::Movement) =>
                            {
                                b = b.color(ORANGE).width(2.0)
                            }
                            val if (self.get_control() == Control::Spell
                                && self.current_spell_index.is_some_and(|i| {
                                    match battle.get_current_wizard().spells[i].spell_input_type() {
                                        SpellInputType::None(damage_filter) => {
                                            damage_filter(battle, (val.0, val.1).into())
                                        }
                                        SpellInputType::Position(
                                            position_filter,
                                            damage_filter,
                                        ) => self.selected_tile.is_some_and(|p1| {
                                            SPELL_POSITION_FILTER[*position_filter](
                                                battle,
                                                p1.into(),
                                            ) && damage_filter(
                                                battle,
                                                p1.into(),
                                                (val.0, val.1).into(),
                                            )
                                        }),
                                        SpellInputType::Direction(damage_filter) => {
                                            self.current_direction.is_some_and(|dir| {
                                                damage_filter(battle, dir, (val.0, val.1).into())
                                            })
                                        }
                                    }
                                })) =>
                            {
                                b = b.color(RED).width(2.0)
                            }
                            val if (self.current_spell_index.is_some_and(|i| {
                                if let super::model::spell::SpellInputType::Position(j, _) =
                                    battle.get_current_wizard().spells[i].spell_input_type()
                                {
                                    SPELL_POSITION_FILTER[*j](battle, (val.0, val.1).into())
                                } else {
                                    false
                                }
                            }) && self.get_control() == Control::Spell)
                                || (self.get_control() == Control::Movement
                                    && battle.wizard_can_move((val.0, val.1).into())) =>
                            {
                                b = b.color(GREEN).width(2.0)
                            }
                            _ => {}
                        }
                        b
                    },
                    ..Default::default()
                })
                .width(Length::Fill)
                .height(Fill)
                .on_press(BattleMessage::TileSelect(x, y))
                .into()
        };

        let board_gen = move || {
            container(
                Column::with_children((0..board::HEIGHT).map(|y| {
                    Row::with_children((0..board::WIDTH).map(|x| cell(x, y)))
                        .spacing(2.0)
                        .into()
                }))
                .spacing(2.0),
            )
        };

        let board = responsive(move |size| {
            let ratio = board::WIDTH as f32 / board::HEIGHT as f32;
            let width = if size.width / size.height < ratio {
                size.width
            } else {
                size.height * ratio
            };
            let height = width / ratio;
            container(board_gen().width(width).height(height))
                .center(Length::Fill)
                .into()
        });

        let spacer = || Space::new(10.0, 10.0);
        column![
            spacer(),
            row![spacer(), container(board), spacer()],
            spacer()
        ]
        .into()
    }

    fn view_battle_info(&self, battle: &Battle) -> Element<BattleMessage> {
        let mut info = Column::with_children([
            Text::new("info:").into(),
            Text::new(format!(
                "x: {}, y: {}",
                self.hovered_tile.0, self.hovered_tile.1
            ))
            .into(),
        ]);
        if let Some(entity) = battle.get_entity_at(self.hovered_tile.into()) {
            info = info.extend(match entity {
                board::Entity::Wizard(w) => {
                    let wiz = battle.get_wizard(w);
                    [
                        Text::new(format!("Team: {}", wiz.team)).into(),
                        Text::new(format!("Health: {}", wiz.health)).into(),
                        Text::new(format!("Mana: {}", wiz.mana)).into(),
                        Text::new(format!(
                            "Effects: {:?}",
                            wiz.effects
                                .iter()
                                .enumerate()
                                .filter(|(_, i)| { **i != 0 })
                                .map(|(i, time)| {
                                    format!(
                                        "{} for {} turns",
                                        Effects::from_repr(i)
                                            .expect("effects cannot be longer than Effects"),
                                        time
                                    )
                                })
                                .collect_vec()
                        ))
                        .into(),
                        Text::new(format!("Spells: {:?}", wiz.spells)).into(),
                    ]
                }
                board::Entity::Projectile(_p) => todo!(),
            });
        }
        container(info.padding(10.0))
            .align_top(Length::Fill)
            .align_left(Length::Fill)
            .into()
    }

    pub fn get_control(&self) -> Control {
        let menu_page = self.control_page.rem_euclid(Control::COUNT as isize) as usize;
        Control::iter()
            .cycle()
            .nth(menu_page)
            .expect("its in a cycle and should never return None")
    }

    fn view_battle_controls(&self, battle: &Battle) -> Element<BattleMessage> {
        let controls = self.get_control();
        let menu_bar: Row<'_, BattleMessage> = Row::with_children([
            button("<")
                .on_press(BattleMessage::ControlPageCycle(false))
                .into(),
            Space::new(FillPortion(1), 0.0).into(),
            container(Into::<&'static str>::into(controls))
                .center_y(Length::Fill)
                .into(),
            Space::new(FillPortion(1), 0.0).into(),
            button(">")
                .on_press(BattleMessage::ControlPageCycle(true))
                .into(),
        ])
        .width(Length::Fill)
        .height(Length::Shrink);

        let control_view = self.view_controls(battle, controls);
        container(column![
            menu_bar,
            container(stack![
                container(control_view)
                    .width(Length::Fill)
                    .height(Length::Fill),
                if self.controls_inputted(battle, controls) {
                    container(column![
                        row![
                            Button::new("Confirm").on_press(BattleMessage::ConfirmAction(controls)),
                            Space::new(Length::Fixed(10.0), Length::Shrink)
                        ],
                        Space::new(Length::Shrink, Length::Fixed(10.0))
                    ])
                    .align_bottom(Length::Fill)
                    .align_right(Length::Fill)
                    .into()
                } else {
                    Into::<Element<BattleMessage>>::into("")
                }
            ])
            .padding(10.0)
        ])
        .align_top(Length::Fill)
        .align_left(Length::Fill)
        .into()
    }

    pub fn view_battle<'a>(&'a self, battle: &'a Battle) -> Element<'a, BattleMessage> {
        container(
            PaneGrid::new(&self.battle_panes, |_pane, pane_type, _focus| {
                let content = match pane_type {
                    super::BattlePane::Battle => container(self.view_board(battle)),
                    super::BattlePane::Info => container(self.view_battle_info(battle)),
                    super::BattlePane::Control => container(self.view_battle_controls(battle)),
                };

                pane_grid::Content::new(
                    container(
                        container(content)
                            .style(|_theme| {
                                container::Style::default()
                                    .background(Background::Color(Color::BLACK))
                            })
                            .width(Length::Fill)
                            .height(Length::Fill),
                    )
                    .padding(2.5)
                    .style(|_theme| container::Style {
                        background: Some(Background::Color(Color::WHITE)),
                        ..Default::default()
                    }),
                )
            })
            .spacing(7.5),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10.0)
        .into()
    }

    pub fn view_spell_select(spell_select: &SpellSelect) -> Element<SpellSelectMessage> {
        stack([
            Row::from_vec(
                spell_select
                    .players
                    .iter()
                    .enumerate()
                    .map(|(i, spell_choice)| {
                        let cell = |text: String| {
                            Container::new(Text::new(text)).center(Length::Fixed(30.0))
                        };
                        let row = |element: SpellElement, text: &'static str, num: usize| {
                            Row::new()
                                .padding(10)
                                .push(
                                    container(Text::new(text).width(Length::Fixed(70.0)))
                                        .center_y(Length::Fixed(30.0)),
                                )
                                .push(Button::new("-").on_press(SpellSelectMessage::PointChange(
                                    PointChange {
                                        player: i,
                                        increment: false,
                                        element,
                                    },
                                )))
                                .push(cell(num.to_string()))
                                .push(Button::new("+").on_press(SpellSelectMessage::PointChange(
                                    PointChange {
                                        player: i,
                                        increment: true,
                                        element,
                                    },
                                )))
                        };
                        container(column![
                            container(
                                Text::new(format!("Player {}", i + 1))
                                    .line_height(LineHeight::Relative(2.0))
                            )
                            .center_x(Length::Fill),
                            container(
                                container(
                                    Column::new()
                                        .padding(5)
                                        .push(
                                            Row::new()
                                                .padding(5)
                                                .push(
                                                    container(Text::new("Unused"))
                                                        .center_y(Length::Fixed(30.0)),
                                                )
                                                .push(cell(spell_choice.unused.to_string())),
                                        )
                                        .push(row(
                                            SpellElement::Water,
                                            "Water:",
                                            spell_choice.water
                                        ))
                                        .push(row(SpellElement::Fire, "Fire:", spell_choice.fire))
                                        .push(row(
                                            SpellElement::Earth,
                                            "Earth:",
                                            spell_choice.earth
                                        ))
                                        .push(row(SpellElement::Wind, "Wind:", spell_choice.wind))
                                        .align_x(Alignment::Center),
                                )
                                .style(|_theme: &Theme| {
                                    container::Style::default()
                                        .background(Background::Color(Color::from_rgb8(10, 10, 10)))
                                        .border(Border::default().width(3).color(Color::WHITE))
                                })
                            )
                            .center(Length::Fill)
                        ])
                        .width(Length::FillPortion(1))
                        .height(Length::Fill)
                        .align_x(Horizontal::Center)
                        .into()
                    })
                    .intersperse_with(|| {
                        container("")
                            .width(Length::Fixed(10.0))
                            .height(Length::Fill)
                            .style(|_| {
                                container::Style::default()
                                    .background(Background::Color(Color::BLACK))
                            })
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
            if spell_select
                .players
                .iter()
                .all(|spell_choice| spell_choice.unused == 0)
            {
                container(column![
                    row![
                        Button::new("Confirm").on_press(SpellSelectMessage::Confirm),
                        Space::new(Length::Fixed(10.0), Length::Shrink)
                    ],
                    Space::new(Length::Shrink, Length::Fixed(10.0))
                ])
                .align_bottom(Length::Fill)
                .align_right(Length::Fill)
                .into()
            } else {
                "".into()
            },
        ])
        .into()
    }
}
