use bevy::prelude::*;

use crate::{
    materials::font::FontMaterials, plugins::combat_mode::CombatMode,
    resources::dictionary::Dictionary,
};

use super::{
    turn_mode::{MovementMode, MovementModeRes},
    ui_root::UserInterfaceRoot,
};

#[derive(Debug, Resource, Copy, Clone)]
pub struct CombatModeData {
    combat_interface_root: Entity,
}

#[derive(Deref, DerefMut, Resource, Clone, Default, Debug)]
pub struct CombatModeRes(CombatMode);

pub fn setup(
    mut commands: Commands,
    ui_root: Res<UserInterfaceRoot>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
) {
    let mut combat_interface_root: Option<Entity> = None;
    commands
        .get_entity(ui_root.entity)
        .unwrap()
        .with_children(|builder| {
            combat_interface_root = Some(
                builder
                    .spawn(NodeBundle::default())
                    .with_children(|builder| {
                        combat_mode_buttons(builder, &font_materials, &dictionary);
                    })
                    .id(),
            );
        });

    commands.insert_resource(CombatModeRes::default());
    commands.insert_resource(CombatModeData {
        combat_interface_root: combat_interface_root.unwrap(),
    });
}

// pub fn setup(
//     mut commands: Commands,
//     font_materials: Res<FontMaterials>,
//     dictionary: Res<Dictionary>,
// ) {
//     let user_interface_root = commands
//         .spawn(NodeBundle {
//             style: Style {
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 position_type: PositionType::Absolute,
//                 ..Default::default()
//             },
//             background_color: BackgroundColor(Color::NONE),
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             combat_mode_buttons(parent, &font_materials, &dictionary);
//         })
//         .insert(Name::new("PlayerUI"))
//         .id();
//
//     commands.insert_resource(CombatModeRes::default());
//     commands.insert_resource(CombatModeData {
//         user_interface_root,
//     });
// }

pub fn combat_mode_buttons(
    root: &mut ChildBuilder,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let font_size = 20.0;

    let glossary = dictionary.get_glossary();
    let ingame_glossary = glossary.combat_mode;

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
        for combat_mode in CombatMode::iterator() {
            let component_name = match *combat_mode {
                CombatMode::InCombat => ingame_glossary.in_combat.clone(),
                CombatMode::OutOfCombat => ingame_glossary.out_of_combat.clone(),
            };

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            top: Val::Px(80.0),
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
                    combat_mode.clone(),
                    Name::new("Combat Mode Button"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(component_name, text_style.clone())
                                .with_alignment(TextAlignment::Center)
                                .with_no_wrap(),
                            ..Default::default()
                        },
                        Name::new("Combat Mode Button Text"),
                    ));
                });
        }
    })
    .insert(Name::new("Combat Mode"));
}

pub fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &CombatMode, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<CombatMode>,
            Without<MovementMode>,
        ),
    >,
    mut current_combat_mode: ResMut<CombatModeRes>,
    mut current_movement_mode: ResMut<MovementModeRes>,
) {
    for (interaction, combat_mode, mut bg_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                match *combat_mode {
                    CombatMode::InCombat => {
                        if **current_combat_mode != CombatMode::InCombat {
                            **current_combat_mode = CombatMode::InCombat;
                            if **current_movement_mode != MovementMode::TurnBasedMovement {
                                **current_movement_mode = MovementMode::TurnBasedMovement;
                            }
                        }
                    }
                    CombatMode::OutOfCombat => {
                        if **current_combat_mode != CombatMode::OutOfCombat {
                            **current_combat_mode = CombatMode::OutOfCombat;
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
