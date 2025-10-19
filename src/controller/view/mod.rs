use iced::{
    alignment::Horizontal,
    widget::{
        column, container, pane_grid, row, stack, text::LineHeight, Button, Column, Container,
        PaneGrid, Row, Space, Text,
    },
    Alignment, Background, Border, Color, Element,
    Length::{self},
    Theme,
};
use itertools::Itertools;

use crate::controller::{message::Message, Controller};

use super::{
    message::{BattleMessage, PointChange, SpellSelectMessage},
    model::{spell::SpellElement, Battle, Model, SpellSelect},
};

impl Controller {
    pub fn view(&self) -> Element<Message> {
        match &self.model {
            Model::Battle(battle) => self.view_battle(battle).map(Message::Battle),
            Model::SpellSelect(spell_select) => {
                Controller::view_spell_select(spell_select).map(Message::SpellSelect)
            }
        }
    }

    pub fn view_battle(&self, battle: &Battle) -> Element<BattleMessage> {
        container(
            PaneGrid::new(&self.battle_panes, |pane, pane_type, _focus| {
                let content = match pane_type {
                    super::BattlePane::Battle => "battke",
                    super::BattlePane::Info => "info",
                    super::BattlePane::Control => "control",
                };

                pane_grid::Content::new(container(content).padding(5.0).style(|_theme| {
                    container::Style {
                        background: Some(Background::Color(Color::BLACK)),
                        ..Default::default()
                    }
                }))
            })
            .spacing(5.0),
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
