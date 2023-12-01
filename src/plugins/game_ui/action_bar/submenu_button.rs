#![allow(dead_code)]

use crate::{
    plugins::game_ui::action_bar::ActionBarButton,
    resources::{dictionary::Dictionary, glossary::Translation},
};
use bevy::prelude::*;
use std::{fmt::Display, slice::Iter};

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum MoveButton {
    #[default]
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

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum AttackButton {
    #[default]
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

impl From<ActionBarButton> for SubMenu {
    fn from(value: ActionBarButton) -> Self {
        match value {
            ActionBarButton::Move => Self::MoveButton,
            ActionBarButton::Attack => Self::AttackButton,
        }
    }
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
                            border: UiRect {
                                left: Val::Px(5.0),
                                right: Val::Px(5.0),
                                top: Val::Px(2.5),
                                bottom: Val::Px(2.5),
                            },
                            ..default()
                        },
                        border_color: Color::BLACK.into(),
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
                        background_color: Color::DARK_GREEN.into(),
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

pub fn handle_submenu_display(
    query_button: Query<(&Interaction, &ActionBarButton)>,
    mut query_submenu: Query<(&mut Style, &Interaction, &SubMenu)>,
) {
    fn handle_interaction(
        interaction: &Interaction,
        sub_interaction: &Interaction,
        mut style: Mut<'_, Style>,
    ) {
        if *interaction == Interaction::Pressed
            || *interaction == Interaction::Hovered
            || *sub_interaction == Interaction::Pressed
            || *sub_interaction == Interaction::Hovered
        {
            style.display = bevy::ui::Display::Flex;
        }
    }

    for (interaction, action_button) in query_button.iter() {
        match action_button {
            ActionBarButton::Move => {
                for (style, sub_interaction, _button) in query_submenu
                    .iter_mut()
                    .filter(|(_, _, submenu)| **submenu == SubMenu::MoveButton)
                {
                    handle_interaction(interaction, sub_interaction, style);
                }
            }
            ActionBarButton::Attack => {
                for (style, sub_interaction, _button) in query_submenu
                    .iter_mut()
                    .filter(|(_, _, submenu)| **submenu == SubMenu::AttackButton)
                {
                    handle_interaction(interaction, sub_interaction, style);
                }
            }
        }
    }
    if query_submenu
        .iter()
        .all(|(_, sub_interaction, _)| *sub_interaction == Interaction::None)
        && query_button
            .iter()
            .all(|(interaction, _)| *interaction == Interaction::None)
    {
        for (mut style, _, _) in query_submenu.iter_mut() {
            style.display = bevy::ui::Display::None;
        }
    }
}

#[derive(Resource, Copy, Clone, Debug, Default)]
pub struct SelectedSubMenu {
    attack_submenu: AttackButton,
    move_submenu: MoveButton,
}

pub fn handle_submenu_buttons(
    mut query_submenu: Query<(
        &mut BackgroundColor,
        &Interaction,
        &SubMenu,
        Option<&MoveButton>,
        Option<&AttackButton>,
    )>,
    mut selected_submenu: ResMut<SelectedSubMenu>,
) {
    for (mut bg_color, interaction, submenu_button, move_button, attack_button) in
        query_submenu.iter_mut()
    {
        match interaction {
            Interaction::Pressed => {
                *bg_color = Color::DARK_GREEN.into();
                match submenu_button {
                    SubMenu::MoveButton => {
                        if let Some(button) = move_button {
                            selected_submenu.move_submenu = *button;
                        }
                    }
                    SubMenu::AttackButton => {
                        if let Some(button) = attack_button {
                            selected_submenu.attack_submenu = *button;
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = Color::GREEN.into();
            }
            Interaction::None => {
                *bg_color = Color::DARK_GREEN.into();
            }
        }
    }
}

pub fn handle_submenu_border(
    mut query_submenu: Query<(
        &mut BorderColor,
        &SubMenu,
        Option<&MoveButton>,
        Option<&AttackButton>,
    )>,
    selected_submenu: Res<SelectedSubMenu>,
) {
    for (mut border_color, submenu, move_button, attack_button) in query_submenu.iter_mut() {
        match submenu {
            SubMenu::MoveButton => {
                if let Some(button) = move_button {
                    if selected_submenu.move_submenu == *button {
                        *border_color = Color::WHITE.into();
                    } else {
                        *border_color = Color::BLACK.into();
                    }
                }
            }
            SubMenu::AttackButton => {
                if let Some(button) = attack_button {
                    if selected_submenu.attack_submenu == *button {
                        *border_color = Color::WHITE.into();
                    } else {
                        *border_color = Color::BLACK.into();
                    }
                }
            }
        }
    }
}
