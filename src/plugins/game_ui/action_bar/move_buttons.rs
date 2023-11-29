#![allow(dead_code)]

use crate::{
    materials::font::FontMaterials,
    plugins::game_ui::action_bar::ActionBarButton,
    resources::{
        dictionary::Dictionary,
        glossary::{MoveSubMenu, Translation},
    },
};
use bevy::prelude::*;
use std::{fmt::Display, slice::Iter};

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum MoveButton {
    MoveAction,
    StandardAction,
    FiveFootStep,
    FullMove,
}

impl MoveButton {
    fn iterator() -> Iter<'static, Self> {
        [
            MoveButton::MoveAction,
            MoveButton::StandardAction,
            MoveButton::FiveFootStep,
            MoveButton::FullMove,
        ]
        .iter()
    }
}

impl Translation for MoveButton {
    fn to_string_glossary(self, glossary: &crate::resources::glossary::Glossary) -> String {
        match self {
            MoveButton::MoveAction => glossary.move_submenu.move_action.clone(),
            MoveButton::StandardAction => glossary.move_submenu.standard_action.clone(),
            MoveButton::FiveFootStep => glossary.move_submenu.five_foot_step.clone(),
            MoveButton::FullMove => glossary.move_submenu.full_move.clone(),
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum AttackButton {
    Single,
    Full,
}

impl AttackButton {
    fn iterator() -> Iter<'static, Self> {
        [AttackButton::Single, AttackButton::Full].iter()
    }
}

impl Translation for AttackButton {
    fn to_string_glossary(self, glossary: &crate::resources::glossary::Glossary) -> String {
        match self {
            AttackButton::Single => glossary.attack_submenu.single_attack.clone(),
            AttackButton::Full => glossary.attack_submenu.full_attack.clone(),
        }
    }
}

impl Display for AttackButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single => write!(f, "Single Attack"),
            Self::Full => write!(f, "Full Attack"),
        }
    }
}

pub fn setup_attack_buttons(
    action_bar_button: &mut ChildBuilder,
    dictionary: &Dictionary,
    text_style: &TextStyle,
    child_dist: f32,
) {
    let glossary = &dictionary.get_glossary();

    action_bar_button
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                bottom: Val::Px(child_dist),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for attack_button in AttackButton::iterator() {
                builder
                    .spawn(ButtonBundle { ..default() })
                    .insert(*attack_button)
                    .insert(Name::from(format!("Attack Button {}", attack_button)))
                    .with_children(|builder| {
                        builder.spawn(TextBundle {
                            text: Text::from_section(
                                attack_button.to_string_glossary(glossary),
                                text_style.clone(),
                            )
                            .with_alignment(TextAlignment::Center)
                            .with_no_wrap(),
                            ..default()
                        });
                    });
            }
        });
}
