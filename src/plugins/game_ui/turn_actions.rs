#![allow(dead_code)]
use bevy::prelude::*;

use crate::{materials::font::FontMaterials, resources::dictionary::Dictionary};

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
            ingame_glossary.move_action,
            ingame_glossary.standard_action,
            ingame_glossary.immediate_action,
            ingame_glossary.five_foot_step,
        ];
        for item in turn_action_items {
            builder.spawn(TextBundle {
                text: Text::from_section(item, text_style.clone())
                    .with_alignment(TextAlignment::Center)
                    .with_no_wrap(),
                background_color: Color::DARK_GREEN.into(),
                style: Style {
                    border: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..Default::default()
            });
        }
    });
}
