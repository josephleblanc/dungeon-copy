use bevy::{prelude::*, utils::HashMap};

// #[derive(Resource, Clone)]
// pub struct MovementQueue {
//     pub start: Vec3,
//     pub end: Vec3,
//     pub queue: Vec<Vec3>,
// }
//
// pub struct MovementMap {
//     pub start: Vec3,
//     pub end: Vec3,
//     pub map: HashMap<Vec2, MapSquare>,
// }
//
// pub fn go_to(
//     mut player_query: Query<(&PlayerComponent, &mut PlayerAnimation, &mut Transform)>,
//     block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
//     keyboard_input: Res<Input<KeyCode>>,
//     time: Res<Time>,
//     mut move_que: ResMut<MovementQueue>,
// ) {
//     let (player_stats, mut player_animation, mut transform) = player_query.single_mut();
//
//     let player_position = transform.translation;
// }
