use bevy::prelude::*;

use crate::{
    materials::font::FontMaterials,
    plugins::game_ui::{combat_mode::combat_mode_buttons, turn_mode::movement_mode_buttons},
    resources::dictionary::Dictionary,
};

use super::{combat_mode::CombatModeRes, turn_mode::MovementModeRes};

#[derive(Resource, Copy, Clone)]
pub struct UserInterfaceRoot {
    pub entity: Entity,
}

pub fn setup(
    mut commands: Commands,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
    //
    assets: Res<AssetServer>,
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
        .with_children(|builder| {
            movement_mode_buttons(builder, &font_materials, &dictionary);
            combat_mode_buttons(builder, &font_materials, &dictionary);
        })
        .insert(Name::new("PlayerUI"))
        .id();

    commands.insert_resource(UserInterfaceRoot {
        entity: user_interface_root,
    });
    commands.insert_resource(MovementModeRes::default());
    commands.insert_resource(CombatModeRes::default());
}
