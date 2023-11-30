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

impl Display for MoveButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveButton::MoveAction => write!(f, "Move Action"),
            MoveButton::StandardAction => write!(f, "Standard Action"),
            MoveButton::FiveFootStep => write!(f, "Five Foot Step"),
            MoveButton::FullMove => write!(f, "Full Move"),
        }
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

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq)]
/// Label struct for the submenu buttons in the action bar.
pub enum SubMenu {
    AttackButton,
    MoveButton,
}

pub fn setup_attack_buttons(
    action_bar_button: &mut ChildBuilder,
    dictionary: &Dictionary,
    text_style: &TextStyle,
    submenu_style: &Style,
) {
    let glossary = &dictionary.get_glossary();

    action_bar_button
        .spawn(NodeBundle {
            style: submenu_style.clone(),
            ..default()
        })
        .insert(Name::from("Attack Button Submenu"))
        .with_children(|builder| {
            for attack_button in AttackButton::iterator() {
                builder
                    .spawn(ButtonBundle {
                        style: Style {
                            display: bevy::ui::Display::None,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(*attack_button)
                    .insert(Name::from(format!("Attack Button {}", attack_button)))
                    .insert(SubMenu::AttackButton)
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

pub fn setup_move_buttons(
    action_bar_button: &mut ChildBuilder,
    dictionary: &Dictionary,
    text_style: &TextStyle,
    submenu_style: &Style,
) {
    let glossary = &dictionary.get_glossary();

    action_bar_button
        .spawn(NodeBundle {
            style: submenu_style.clone(),
            ..default()
        })
        .insert(Name::from("Move Button Submenu"))
        .with_children(|builder| {
            for move_button in MoveButton::iterator() {
                builder
                    .spawn(ButtonBundle {
                        style: Style {
                            display: bevy::ui::Display::None,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(*move_button)
                    .insert(Name::from(format!("Move Button {}", move_button)))
                    .insert(SubMenu::MoveButton)
                    .with_children(|builder| {
                        builder.spawn(TextBundle {
                            text: Text::from_section(
                                move_button.to_string_glossary(glossary),
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

pub fn handle_submenu_buttons(
    query_button: Query<(&Interaction, &ActionBarButton)>,
    mut query_submenu: Query<(&mut Style, &Interaction, &SubMenu)>,
) {
    for (interaction, action_button) in query_button.iter() {
        if let Some((mut style, sub_interaction, _button)) = match action_button {
            ActionBarButton::Move => query_submenu
                .iter_mut()
                .find(|(_, _, submenu)| **submenu == SubMenu::MoveButton),
            ActionBarButton::Attack => query_submenu
                .iter_mut()
                .find(|(_, _, submenu)| **submenu == SubMenu::AttackButton),
        } {
            if *interaction == Interaction::Pressed
                || *interaction == Interaction::Hovered
                || *sub_interaction == Interaction::Pressed
                || *sub_interaction == Interaction::Hovered
            {
                style.display = bevy::ui::Display::Flex;
            } else if *interaction == Interaction::None && style.display != bevy::ui::Display::None
            {
                style.display = bevy::ui::Display::None;
            }
        }
    }
}
