use bevy::prelude::*;

#[derive(Component)]
pub struct UserInterfaceCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_user_interface_camera);
    }
}

fn spawn_user_interface_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(Name::new("UserInterfaceCamera"))
        .insert(UserInterfaceCamera);
}
