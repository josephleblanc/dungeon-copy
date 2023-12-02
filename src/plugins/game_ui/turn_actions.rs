#![allow(dead_code)]
use bevy::prelude::*;
use std::slice::Iter;

use crate::{
    materials::font::FontMaterials,
    plugins::{
        actions::{event::TurnActionEvent, TurnAction, TurnActionStatus},
        combat_mode::turn::action::CurrentTurn,
        player::PlayerLabel,
    },
    resources::dictionary::Dictionary,
};

use super::ui_root::UserInterfaceRoot;

#[derive(Debug, Resource, Copy, Clone)]
pub struct TurnActionData {
    turn_action_interface_root: Entity,
}

pub fn setup(
    mut commands: Commands,
    ui_root: Res<UserInterfaceRoot>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
) {
    let mut turn_action_interface_root: Option<Entity> = None;
    commands
        .get_entity(ui_root.entity)
        .unwrap()
        .with_children(|builder| {
            turn_action_interface_root = Some(
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        turn_action_display(builder, &font_materials, &dictionary);
                    })
                    .id(),
            );
        });

    commands.insert_resource(TurnActionData {
        turn_action_interface_root: turn_action_interface_root.unwrap(),
    });
}

fn turn_action_display(
    root: &mut ChildBuilder,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let font_size = 20.0;

    let glossary = dictionary.get_glossary();
    let ingame_glossary = glossary.turn_action_display;

    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        color: Color::WHITE,
    };

    root.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            height: Val::Auto,
            width: Val::Auto,
            flex_direction: FlexDirection::ColumnReverse,
            border: UiRect::all(Val::Px(8.0)),
            row_gap: Val::Px(8.0),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        },
        // background_color: Color::Transpar.into(),
        ..default()
    })
    .with_children(|builder| {
        let turn_action_items = [
            (ingame_glossary.move_action, TurnActionButton::Move),
            (ingame_glossary.standard_action, TurnActionButton::Standard),
            (
                ingame_glossary.immediate_action,
                TurnActionButton::Immediate,
            ),
            (
                ingame_glossary.five_foot_step,
                TurnActionButton::FiveFootStep,
            ),
        ];
        builder
            .spawn(ButtonBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    margin: UiRect::top(Val::Px(8.0)),
                    padding: UiRect {
                        left: Val::Px(6.0),
                        right: Val::Px(6.0),
                        top: Val::Px(4.0),
                        bottom: Val::Px(4.0),
                    },
                    ..default()
                },
                background_color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
                ..default()
            })
            .insert(Name::from("Attacks of Opportunity"))
            .with_children(|builder| {
                builder.spawn(TextBundle {
                    text: Text::from_section("AoO: ", text_style.clone())
                        .with_alignment(TextAlignment::Center)
                        .with_no_wrap(),
                    ..Default::default()
                });
                builder
                    .spawn(TextBundle {
                        text: Text::from_section("1", text_style.clone())
                            .with_alignment(TextAlignment::Center)
                            .with_no_wrap(),
                        ..Default::default()
                    })
                    .insert(AOOLabel(1));
            });
        for (text, component) in turn_action_items {
            builder
                .spawn(ButtonBundle {
                    background_color: Color::DARK_GREEN.into(),
                    style: Style {
                        padding: UiRect {
                            left: Val::Px(6.0),
                            right: Val::Px(6.0),
                            top: Val::Px(4.0),
                            bottom: Val::Px(4.0),
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(component)
                .insert(Name::from(format!("Turn Action Item: {}", text)))
                .with_children(|builder| {
                    builder.spawn(TextBundle {
                        text: Text::from_section(text, text_style.clone())
                            .with_alignment(TextAlignment::Center)
                            .with_no_wrap(),
                        ..Default::default()
                    });
                });
        }
    });
}

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub struct AOOLabel(usize);

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TurnActionButton {
    Move,
    Standard,
    FiveFootStep,
    Immediate,
    FullRound,
}

impl TurnActionButton {
    const GREEN_BUTTON: Color = Color::rgba(0.5, 0.5, 0.0, 0.9);
    pub fn update_color(
        mut query_button: Query<(&Self, &mut BackgroundColor), With<Button>>,
        mut turn_action_event: EventReader<TurnActionEvent>,
    ) {
        for event in turn_action_event.into_iter() {
            match event.status {
                TurnActionStatus::Used => {
                    let (_button, mut bg_color) = query_button
                        .iter_mut()
                        .find(|(button, _bg_color)| event.turn_action == (**button).into())
                        .unwrap();
                    *bg_color = Color::rgba(0.5, 0.0, 0.0, 0.9).into();
                }
                TurnActionStatus::Available => {
                    let (_button, mut bg_color) = query_button
                        .iter_mut()
                        .find(|(button, _bg_color)| event.turn_action == (**button).into())
                        .unwrap();
                    *bg_color = Self::GREEN_BUTTON.into();
                }
                TurnActionStatus::Planned => {
                    let (_button, mut bg_color) = query_button
                        .iter_mut()
                        .find(|(button, _bg_color)| event.turn_action == (**button).into())
                        .unwrap();
                    *bg_color = Color::rgba(0.5, 0.5, 0.0, 0.9).into();
                }
            }
        }
    }

    pub fn reset_color(
        mut query_button: Query<&mut BackgroundColor, With<Button>>,
        current_turn: Res<CurrentTurn>,
        query_player: Query<Entity, With<PlayerLabel>>,
    ) {
        if current_turn.is_changed() && query_player.get(current_turn.entity).is_ok() {
            for mut bg_color in query_button.iter_mut() {
                *bg_color = Self::GREEN_BUTTON.into();
            }
        }
    }
}
