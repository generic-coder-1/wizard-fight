use crate::controller::{
    message::BattleMessage,
    model::{
        position::Direction,
        spell::{SpellInputType, SPELL_POSITION_FILTER},
        Battle,
    },
    Controller,
};
use iced::{
    widget::{button, column, container, responsive, row, tooltip, Column, Row, Space, Text},
    Alignment::Center,
    Background, Border, Color, Element, Length, Theme,
};
use itertools::Itertools;
use strum_macros::{EnumCount, EnumIter, EnumString, IntoStaticStr};

use super::{GREY, YELLOW};

#[derive(EnumIter, EnumCount, EnumString, IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Control {
    Movement,
    Spell,
}

impl Controller {
    pub fn view_controls(&self, battle: &Battle, control: Control) -> Element<BattleMessage> {
        let wizard = battle.get_current_wizard();
        let d_pad = || {
            responsive(move |size| {
                let length = size.width.min(size.height);
                let spacer = || Space::new(Length::FillPortion(1), Length::FillPortion(1));
                let button_maker = |text, dir| {
                    let border = if self
                        .current_direction
                        .is_some_and(|curr_dir| curr_dir == dir)
                    {
                        Border::default().width(3).color(YELLOW)
                    } else {
                        Border::default()
                    };
                    button(
                        Text::new(text)
                            .align_x(Center)
                            .align_y(Center)
                            .width(Length::Fill)
                            .height(Length::Fill),
                    )
                    .on_press(BattleMessage::DirectionSelect(dir))
                    .width(Length::FillPortion(1))
                    .height(Length::FillPortion(1))
                    .style(move |theme: &Theme, status| {
                        let mut b = button::Catalog::style(
                            theme,
                            &<Theme as button::Catalog>::default(),
                            status,
                        );
                        b.border = border;
                        b
                    })
                };
                container(row![
                    column![spacer(), button_maker("←", Direction::Left), spacer()],
                    column![
                        button_maker("↑", Direction::Up),
                        spacer(),
                        button_maker("↓", Direction::Down)
                    ],
                    column![spacer(), button_maker("→", Direction::Right), spacer()],
                ])
                .width(length)
                .height(length)
                .into()
            })
        };

        match control {
            Control::Movement => Column::new()
                .push(Text::new(if let Some((x, y)) = self.selected_tile {
                    format!("x: {x}, y: {y}")
                } else {
                    "No Position Selected".to_owned()
                }))
                .into(),
            Control::Spell => {
                let mut spell_controls = Row::new();
                //spell selecting
                spell_controls = spell_controls
                    .push(column![
                        Column::with_children(
                            wizard
                                .spells
                                .iter()
                                .enumerate()
                                .map(|(i, spell)| -> Element<BattleMessage> {
                                    tooltip(
                                        button(Text::new(i.to_string()))
                                            .on_press(BattleMessage::SpellChoose(i)),
                                        container(Text::new(format!("{spell:?}")))
                                            .padding(2)
                                            .style(|_| {
                                                container::Style::default()
                                                    .background(Background::Color(GREY))
                                                    .border(
                                                        Border::default()
                                                            .width(2)
                                                            .color(Color::BLACK),
                                                    )
                                            }),
                                        tooltip::Position::Left,
                                    )
                                    .into()
                                })
                                .intersperse_with(|| Space::with_height(5.0).into()),
                        )
                        .height(Length::FillPortion(1)),
                        Space::with_height(Length::FillPortion(1))
                    ])
                    .push(Space::with_width(10.0)); //padding
                                                    //spell control info
                if let Some(spell_index) = self.current_spell_index {
                    let spell = wizard.spells[spell_index];
                    let control_info: Element<BattleMessage> = match spell.spell_input_type() {
                        SpellInputType::None(_) => container("").into(),
                        SpellInputType::Position(_, _) => {
                            column![Text::new(if let Some((x, y)) = self.selected_tile {
                                format!("x: {x}, y: {y}")
                            } else {
                                "No Position Selected".to_owned()
                            })]
                            .into()
                        }
                        SpellInputType::Direction(_) => d_pad().into(),
                    };
                    spell_controls =
                        spell_controls.push(column![Text::new(format!("{spell:?}")), control_info]);
                };

                spell_controls.into()
            }
        }
    }

    pub fn controls_inputted(&self, battle: &Battle, control: Control) -> bool {
        match control {
            Control::Movement => self
                .selected_tile
                .is_some_and(|pos| battle.wizard_can_move(pos.into())),
            Control::Spell => self.current_spell_index.is_some_and(|i| {
                match battle.get_current_wizard().spells[i].spell_input_type() {
                    SpellInputType::None(_) => true,
                    SpellInputType::Position(j, _) => self
                        .selected_tile
                        .is_some_and(|tile| SPELL_POSITION_FILTER[*j](battle, tile.into())),
                    SpellInputType::Direction(_) => self.current_direction.is_some(),
                }
            }),
        }
    }
}
