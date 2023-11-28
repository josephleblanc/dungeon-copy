#![allow(dead_code)]

use bevy::prelude::*;
use std::slice::Iter;

use crate::{
    materials::font::FontMaterials,
    resources::{dictionary::Dictionary, glossary::ActionBar},
};

use super::ui_root::UserInterfaceRoot;

#[derive(Resource, Debug, Clone, Copy)]
pub struct ActionBarData {
    action_bar_root: Entity,
}

pub fn setup(
    mut commands: Commands,
    ui_root: Res<UserInterfaceRoot>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
) {
    let mut action_bar_root: Option<Entity> = None;
    commands
        .get_entity(ui_root.entity)
        .unwrap()
        .with_children(|builder| {
            action_bar_root = Some(
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            left: Val::Percent(15.0),
                            width: Val::Percent(70.0),
                            height: Val::Percent(15.0),
                            ..default()
                        },
                        background_color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
                        ..default()
                    })
                    .with_children(|builder| {
                        action_bar(builder, &font_materials, &dictionary);
                    })
                    .insert(Name::from("Action Bar Root"))
                    .id(),
            );
        });

    commands.insert_resource(ActionBarData {
        action_bar_root: action_bar_root.unwrap(),
    });
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ActionBarButton {
    Attack,
    Move,
}

impl ActionBarButton {
    pub fn iterator() -> Iter<'static, Self> {
        [ActionBarButton::Attack, ActionBarButton::Move].iter()
    }

    pub fn to_string_glossary(self, action_bar: &ActionBar) -> String {
        match self {
            Self::Attack => action_bar.attack.clone(),
            Self::Move => action_bar.move_action.clone(),
        }
    }
}

pub fn action_bar(
    root: &mut ChildBuilder,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let font_size = 22.0;

    let glossary = dictionary.get_glossary();
    let ingame_glossary = glossary.action_bar;

    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        color: Color::WHITE,
    };

    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        for action_button in ActionBarButton::iterator() {
            let component_name = action_button.to_string_glossary(&ingame_glossary);

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            border: UiRect::all(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(15.)),
                            margin: UiRect::all(Val::Px(15.)),
                            ..Default::default()
                        },
                        background_color: Color::DARK_GREEN.into(),
                        border_color: Color::BLACK.into(),
                        ..Default::default()
                    },
                    *action_button,
                    Name::new("Action Button"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(component_name, text_style.clone())
                                .with_alignment(TextAlignment::Center)
                                .with_no_wrap(),
                            ..Default::default()
                        },
                        Name::new("Action Button Text"),
                    ));
                });
        }
    })
    .insert(Name::new("Movement Mode"));
}
