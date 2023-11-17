use bevy::prelude::*;

#[derive(Resource, Copy, Clone)]
pub struct UserInterfaceRoot {
    pub entity: Entity,
}

pub fn setup(mut commands: Commands) {
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
        .insert(Name::new("PlayerUI"))
        .id();

    commands.insert_resource(UserInterfaceRoot {
        entity: user_interface_root,
    });
}
