use bevy::prelude::*;
use std::slice::Iter;

use crate::components::player::PlayerComponent;
// use crate::components::skill::SkillComponent;
use crate::config::{RESOLUTION, WINDOW_HEIGHT};
use crate::materials::font::FontMaterials;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::game_ui::IngameUiData;
use crate::resources::dictionary::Dictionary;
// use crate::resources::skill::skill_type::SkillType;

#[derive(Component, Clone)]
pub enum MovementModeText {
    WanderMovement,
    TurnBasedMovement,
}

impl MovementModeText {
    pub fn iterator() -> Iter<'static, MovementModeText> {
        [
            MovementModeText::WanderMovement,
            MovementModeText::TurnBasedMovement,
        ]
        .iter()
    }
}

#[derive(Component)]
pub struct WanderMovement {}

#[derive(Component)]
pub struct TurnBasedMovement {}

pub fn setup(
    mut commands: Commands,
    font_materials: Res<FontMaterials>,
    ingame_materials: Res<InGameMaterials>,
    dictionary: Res<Dictionary>,
) {
    let user_interface_root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            movement_mode(parent, &font_materials, &dictionary);
        })
        .insert(Name::new("PlayerUI"))
        .id();
}

pub fn cleanup(mut commands: Commands, ingame_ui_data: Res<IngameUiData>) {
    commands
        .entity(ingame_ui_data.user_interface_root)
        .despawn_recursive();
}

pub fn movement_mode(
    root: &mut ChildBuilder,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let font_size = 20.0;

    let glossary = dictionary.get_glossary();
    let ingame_glossary = glossary.movement_mode_text;

    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        color: Color::WHITE.into(),
    };

    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        for (index, movement_mode_text) in MovementModeText::iterator().enumerate() {
            let component_name = match *movement_mode_text {
                MovementModeText::WanderMovement => ingame_glossary.wander_movement.clone(),
                MovementModeText::TurnBasedMovement => ingame_glossary.turn_based_movement.clone(),
            };

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            top: Val::Px(10.0),
                            height: Val::Px(60.0),
                            border: UiRect::all(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(15.)),
                            margin: UiRect::all(Val::Px(15.)),
                            ..Default::default()
                        },
                        background_color: Color::DARK_GREEN.into(),
                        border_color: Color::BLACK.into(),
                        ..Default::default()
                    },
                    Name::new("Movement Mode Button"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(component_name, text_style.clone())
                                .with_alignment(TextAlignment::Center)
                                .with_no_wrap(),
                            ..Default::default()
                        },
                        Name::new("Movement Mode Button Text"),
                    ));
                });
        }
    })
    .insert(Name::new("Movement Mode"));
}
