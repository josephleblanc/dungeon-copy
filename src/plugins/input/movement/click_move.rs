use bevy::prelude::*;

use crate::plugins::input::movement::Movement;
use crate::plugins::input::movement::MovementMode;
use crate::plugins::input::movement::MovementModeRes;
use crate::plugins::input::movement::PlayerAnimation;
use crate::plugins::input::movement::PlayerComponent;
use crate::plugins::interact::Interactable;
use crate::plugins::interact::InteractingPos;
use crate::plugins::player::collisions::wall_collision_check;
use crate::resources::dungeon::block_type::BlockType;
use crate::resources::dungeon::grid_square::GridSquare;

#[derive(Resource, Clone, Debug)]
pub struct MovementPath {
    pub queue: Vec<(Vec3, Vec3)>,
}

#[derive(Clone, Debug)]
pub struct MoveNode {
    pub pos: Vec3,
    pub dist: f32,
    pub open: bool,
    pub path: Vec<Vec3>,
}

#[derive(Event, Debug)]
pub struct MovementPathEvent {
    move_path: MovementPath,
    insert: bool,
}

impl MoveNode {
    pub fn to_new_pos(&self, step: Vec3, dest: Vec3) -> MoveNode {
        let mut path: Vec<Vec3> = self.path.clone();
        path.push(self.pos + step);
        MoveNode {
            pos: self.pos + step,
            dist: (self.pos + step).distance(dest),
            open: true,
            path,
        }
    }
}

pub fn handle_click(
    button: Res<Input<MouseButton>>,
    player_query: Query<(&PlayerComponent, &Transform)>,
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    mut event_writer: EventWriter<MovementPathEvent>,
    interacting_pos: Res<InteractingPos>,
    movement_mode: Res<MovementModeRes>,
    grid_square_query: Query<(&GridSquare, &Interactable), Without<PlayerComponent>>,
    movement: Res<Movement>,
) {
    // TODO: Make sure player is in a grid square position before calling set_path
    if button.just_pressed(MouseButton::Left)
        && interacting_pos.active
        && **movement_mode == MovementMode::TurnBasedMovement
        && !movement.moving
    {
        let (_player_stats, player_pos) = player_query.get_single().unwrap();
        let player_pos_2d = player_pos.translation.truncate();
        if grid_square_query
            .iter()
            .any(|(_grid_square, interactable)| interactable.center == player_pos_2d)
        {
            set_path(
                player_query,
                block_type_query,
                event_writer,
                interacting_pos,
            );
        }
    }
}

pub fn set_path(
    player_query: Query<(&PlayerComponent, &Transform)>,
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    mut event_writer: EventWriter<MovementPathEvent>,
    interacting_pos: Res<InteractingPos>,
) {
    println!("debug | set_path | start");
    let (_player, transform) = player_query.single();

    let start = transform.translation;
    let end = Vec3::new(interacting_pos.pos.x, interacting_pos.pos.y, start.z);
    println!("debug | set_path | variable start = {:?}", start);
    println!("debug | set_path | variable end = {:?}", end);

    let start_node = MoveNode {
        pos: start,
        dist: start.distance(end),
        open: true,
        path: vec![start],
    };

    let mut open_paths: Vec<MoveNode> = vec![start_node.clone()];

    while open_paths.iter().any(|node| node.open) {
        let closest = open_paths
            .iter_mut()
            .filter(|node| node.open)
            .min_by(|x, y| x.dist.total_cmp(&y.dist))
            .unwrap();
        if closest.dist == 0.0 {
            let queue: Vec<(Vec3, Vec3)> = closest
                .path
                .clone()
                .into_iter()
                .zip(closest.path.clone().into_iter().skip(1))
                .rev()
                .collect();
            event_writer.send(MovementPathEvent {
                move_path: MovementPath { queue },
                insert: true,
            });
            return;
        }
        let possible_paths =
            wall_collision_check(start, &block_type_query).open_nodes(closest, end);
        closest.open = false;

        open_paths.extend_from_slice(&possible_paths);
    }
}

pub fn move_path_system(
    mut player_query: Query<(&PlayerComponent, &mut PlayerAnimation, &mut Transform)>,
    move_que: Option<ResMut<MovementPath>>,
    mut movement: ResMut<Movement>,
    time: Res<Time>,
    mut event_writer: EventWriter<MovementPathEvent>,
) {
    let (player_stats, _player_animation, mut transform) = player_query.single_mut();
    if !movement.moving {
        if let Some(mut move_que) = move_que {
            // let (player_stats, mut player_animation, mut transform) = player_query.single_mut();

            if let Some((start, end)) = move_que.queue.pop() {
                let delta = end - start;
                movement.set_target(
                    transform.translation.truncate(),
                    delta.truncate(),
                    player_stats.speed,
                );
            } else {
                event_writer.send(MovementPathEvent {
                    move_path: move_que.clone(),
                    insert: false,
                });
            }
        }
    } else if !movement.is_finished() {
        let time_delta = time.delta();
        movement
            .update(&mut transform.translation, time_delta)
            .unwrap();
        if movement.is_finished() {
            movement.reset();
        }
    }
}

pub fn move_event_system(
    mut commands: Commands,
    move_que: Option<Res<MovementPath>>,
    mut event_reader: EventReader<MovementPathEvent>,
    movement_mode: Res<MovementModeRes>,
) {
    if **movement_mode == MovementMode::TurnBasedMovement {
        // println!("debug | mouse_event_system | start");
        if let Some(event) = event_reader.into_iter().next() {
            println!("debug | mouse_event_system | reading event");
            if event.insert {
                println!("debug | mouse_event_system | inserting resource");
                commands.insert_resource::<MovementPath>(event.move_path.to_owned());
            } else {
                if let Some(move_que) = move_que {
                    println!("debug | mouse_event_system | checking if move_que is empty");
                    if move_que.queue.is_empty() {
                        println!("debug | mouse_event_system | removing resource MovementPath");
                        commands.remove_resource::<MovementPath>();
                    }
                }
            }
        }
    }
}
