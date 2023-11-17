use bevy::prelude::*;
use std::ops::Deref;
use std::ops::DerefMut;
use std::slice::Iter;

use crate::components::player::PlayerComponent;
// use crate::components::skill::SkillComponent;
use crate::config::{RESOLUTION, WINDOW_HEIGHT};
use crate::materials::font::FontMaterials;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::combat_mode::CombatMode;
// use crate::plugins::game_ui::IngameUiData;
use crate::resources::dictionary::Dictionary;

use super::ui_root::UserInterfaceRoot;
// use crate::resources::skill::skill_type::SkillType;

#[derive(Resource)]
pub struct MovementModeData {
    movement_interface_root: Entity,
}

#[derive(Resource, Clone, Default, Debug)]
pub struct MovementModeRes(MovementMode);

#[derive(Component, Clone, Default, Resource, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MovementMode {
    #[default]
    WanderMovement,
    TurnBasedMovement,
}

impl MovementMode {
    pub fn iterator() -> Iter<'static, MovementMode> {
        [
            MovementMode::WanderMovement,
            MovementMode::TurnBasedMovement,
        ]
        .iter()
    }
}

impl Deref for MovementModeRes {
    type Target = MovementMode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MovementModeRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component)]
pub struct WanderMovement {}

#[derive(Component)]
pub struct TurnBasedMovement {}

pub fn setup(
    mut commands: Commands,
    ui_root: Res<UserInterfaceRoot>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
) {
    let mut movement_interface_root: Option<Entity> = None;
    commands
        .get_entity(ui_root.entity)
        .unwrap()
        .with_children(|builder| {
            movement_interface_root = Some(
                builder
                    .spawn(NodeBundle::default())
                    .with_children(|builder| {
                        movement_mode_buttons(builder, &font_materials, &dictionary);
                    })
                    .id(),
            );
        });

    commands.insert_resource(MovementModeRes::default());
    commands.insert_resource(MovementModeData {
        movement_interface_root: movement_interface_root.unwrap(),
    });
}

pub fn movement_mode_buttons(
    root: &mut ChildBuilder,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let font_size = 20.0;

    let glossary = dictionary.get_glossary();
    let ingame_glossary = glossary.movement_mode;

    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        color: Color::WHITE,
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
        for movement_mode in MovementMode::iterator() {
            let component_name = match *movement_mode {
                MovementMode::WanderMovement => ingame_glossary.wander_movement.clone(),
                MovementMode::TurnBasedMovement => ingame_glossary.turn_based_movement.clone(),
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
                    movement_mode.clone(),
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

pub fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &MovementMode, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<MovementMode>,
            Without<CombatMode>,
        ),
    >,
    mut current_mode: ResMut<MovementModeRes>,
) {
    for (interaction, movement_mode, mut bg_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                match *movement_mode {
                    MovementMode::TurnBasedMovement => {
                        if **current_mode != MovementMode::TurnBasedMovement {
                            **current_mode = MovementMode::TurnBasedMovement;
                        }
                    }
                    MovementMode::WanderMovement => {
                        if **current_mode != MovementMode::WanderMovement {
                            **current_mode = MovementMode::WanderMovement;
                        }
                    }
                }
                *bg_color = Color::DARK_GREEN.into();
            }
            Interaction::Hovered => *bg_color = Color::GREEN.into(),
            Interaction::None => *bg_color = Color::DARK_GREEN.into(),
        };
    }
}
