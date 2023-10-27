use std::ops::ControlFlow;
use std::ops::Deref;
use std::ops::DerefMut;

use bevy::prelude::*;

use crate::events::MovePathEvent;
// use crate::plugins::game_ui::map::pathing::despawn_move_path;
use crate::plugins::game_ui::map::pathing::spawn_move_path;
use crate::plugins::game_ui::map::pathing::MovePathFrameData;
use crate::plugins::game_ui::map::pathing::PathSpriteEvent;
use crate::plugins::game_ui::map::pathing::SpriteAction;
use crate::plugins::input::movement::Movement;
use crate::plugins::input::movement::MovementMode;
use crate::plugins::input::movement::MovementModeRes;
use crate::plugins::input::movement::PlayerAnimation;
use crate::plugins::input::movement::PlayerComponent;
use crate::plugins::interact::Interactable;
use crate::plugins::interact::InteractingPos;
use crate::plugins::interact::InteractingPosEvent;
use crate::plugins::player::collisions::wall_collision_check;
use crate::resources::dungeon::block_type::BlockType;
use crate::resources::dungeon::grid_square::GridSquare;

use super::map::MapGrid;

#[derive(Resource, Clone, Debug)]
pub struct MovementPath {
    pub path: Vec<(Vec3, Vec3)>,
    active: bool,
}

impl MovementPath {
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self) {
        self.active = true;
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
    }

    pub fn new(path: Vec<(Vec3, Vec3)>) -> Self {
        Self {
            path,
            active: false,
        }
    }

    pub fn new_active(path: Vec<(Vec3, Vec3)>) -> Self {
        Self { path, active: true }
    }

    pub fn to_event(self, action: MovePathAction) -> MovementPathEvent {
        MovementPathEvent {
            move_path: Some(self),
            action: Some(action),
        }
    }
}

#[derive(Resource, Clone, Debug)]
pub struct MovementPathList {
    pub list: Vec<MovementPath>,
    pub shortest: MovementPath,
    pub focused: MovementPath,
    pub start: Vec3,
    pub end: Vec3,
    pub displayed: bool,
}

impl MovementPathList {
    /// Creates a new MovementPathList from a MovementPath. By default the new
    /// MovementPathList will have a `shortest` and `focused` of clones of the
    /// new path.
    pub fn new_from_path(path: MovementPath) -> Self {
        let (start, _) = *path.path.first().unwrap();
        let (_, end) = *path.path.last().unwrap();
        MovementPathList {
            list: vec![path.clone()],
            shortest: path.clone(),
            focused: path.clone(),
            start,
            end,
            displayed: false,
        }
    }

    /// Adds a new path to the list. Consumes by default, clone if needed.
    pub fn add_to_paths(&mut self, path: MovementPath) {
        self.list.push(path);
    }

    /// Set a path as active. If the path is not found in the list, the new path
    /// will be added and set as the focused path.
    /// If the new_focused path is set to active, this will set them to inactive
    /// when it is made focused.
    pub fn set_focused(&mut self, mut new_focused: MovementPath) {
        let is_contained = self.list.iter().any(|path| path.path == new_focused.path);

        new_focused.set_inactive();
        if is_contained {
            self.focused = new_focused;
        } else {
            self.list.push(new_focused.clone());
            self.focused = new_focused;
        }
    }

    pub fn add_focused(&mut self, mut new_focused: MovementPath) {
        new_focused.set_inactive();
        self.list.push(new_focused.clone());
        self.focused = new_focused;
    }
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
    move_path: Option<MovementPath>,
    action: Option<MovePathAction>,
}

#[derive(Clone, Debug, Copy)]
pub enum MovePathAction {
    Insert,
    Remove,
    // TakePath,
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

pub fn handle_path(
    path_list: Option<Res<MovementPathList>>,
    button: Res<Input<MouseButton>>,
    mut list_event_writer: EventWriter<PathListEvent>,
    mut move_event_writer: EventWriter<MovementPathEvent>,
    mut interact_event_reader: EventReader<InteractingPosEvent>,

    interacting_pos: Res<InteractingPos>,
    movement_mode: Res<MovementModeRes>,
    movement: Res<Movement>,
    player_query: Query<(&PlayerComponent, &Transform)>,
    map_grid: Res<MapGrid>,
) {
    let debug = false;
    // if debug {
    //     println!("debug | handle_path | start handle_path");
    // }

    let player_pos = player_query.get_single().unwrap().1.translation.truncate();
    let focus_pos = interacting_pos.pos;
    if interacting_pos.active
        && **movement_mode == MovementMode::TurnBasedMovement
        && !movement.moving
        && player_pos != focus_pos
        && map_grid.positions.as_slice().contains(&player_pos)
    {
        if debug {
            println!("debug | handle_path |     player_pos = {:?}", player_pos);
            println!(
                "debug | handle_path |     interacting_pos = {:?}",
                focus_pos
            );
            println!(
                "debug | handle_path | map_grid.positions.as_slice().contains(&player_pos): {}",
                map_grid.positions.as_slice().contains(&player_pos)
            );
        }
        if let Some(path_list) = path_list {
            if button.just_pressed(MouseButton::Left) {
                move_event_writer.send(path_list.focused.clone().to_event(MovePathAction::Insert))
            } else if button.just_pressed(MouseButton::Right) {
                list_event_writer.send(PathAction::Repath.into());
            } else {
                if interacting_pos.is_changed() {
                    println!("interacting_pos changed");
                }
                for _event in interact_event_reader.into_iter() {
                    println!("Sending removed");
                    list_event_writer.send(PathAction::Remove.into());
                }
            }
        } else {
            println!("sending start_path");
            list_event_writer.send(PathAction::StartPath.into());
        }
    }
}

#[derive(Resource, Clone, Default)]
pub struct Paths {
    pub paths: Vec<MoveNode>,
    pub ignore: Vec<MoveNode>,
}

impl Paths {
    pub fn ignore_node(&mut self, node: MoveNode) {
        self.ignore.push(node);
    }
}

impl Deref for Paths {
    type Target = Vec<MoveNode>;
    fn deref(&self) -> &Self::Target {
        &self.paths
    }
}

impl DerefMut for Paths {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.paths
    }
}

impl Paths {
    pub fn reset(&mut self) {
        self.paths = vec![];
    }
}

// pub fn set_path(
//     player_query: Query<(&PlayerComponent, &Transform)>,
//     block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
//     mut event_writer: EventWriter<MovementPathEvent>,
//     interacting_pos: Res<InteractingPos>,
//     mut open_paths: ResMut<Paths>,
// ) {
//     println!("debug | set_path | start");
//     let (_player, transform) = player_query.single();
//
//     let start = transform.translation;
//     let end = Vec3::new(interacting_pos.pos.x, interacting_pos.pos.y, start.z);
//     println!("debug | set_path | variable start = {:?}", start);
//     println!("debug | set_path | variable end = {:?}", end);
//
//     let start_node = MoveNode {
//         pos: start,
//         dist: start.distance(end),
//         open: true,
//         path: vec![start],
//     };
//
//     if open_paths.paths.is_empty() {
//         // let no: () = open_paths.paths;
//         open_paths.paths = vec![start_node];
//     }
//
//     let finished_path = find_path(open_paths, block_type_query, end).unwrap();
//
//     event_writer.send(MovementPathEvent {
//         move_path: Some(finished_path),
//         action: Some(MovePathAction::Insert),
//     });
// }

pub fn start_path_list(
    mut commands: Commands,
    player_query: Query<(&PlayerComponent, &Transform)>,
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    interacting_pos: Res<InteractingPos>,
    mut open_paths: ResMut<Paths>,
    mut sprite_writer: EventWriter<PathSpriteEvent>,
    mut event_reader: EventReader<PathListEvent>,
) {
    open_paths.reset();
    let debug = true;
    if debug {
        println!("debug | start_path_list | start start_path_list");
    }
    if event_reader
        .into_iter()
        .filter_map(|event| event.action)
        // .inspect(|action| {
        //     println!(
        //         "debug | start_path_list | event action in event iterator: {:?}",
        //         action
        //     );
        // })
        .any(|action| action == PathAction::StartPath)
    {
        if debug {
            println!("debug | start_path_list | inside event condition if block");
        }
        let (_player, transform) = player_query.single();

        let start = transform.translation;
        let end = Vec3::new(interacting_pos.pos.x, interacting_pos.pos.y, start.z);
        if debug {
            println!("debug | start_path_list | variable start = {:?}", start);
            println!("debug | start_path_list | variable end = {:?}", end);
        }

        let start_node = MoveNode {
            pos: start,
            dist: start.distance(end),
            open: true,
            path: vec![start],
        };

        if open_paths.paths.is_empty() {
            open_paths.paths = vec![start_node];
        }

        let finished_path = find_path(open_paths, block_type_query, end).unwrap();

        if debug {
            println!(
                "debug | start_path_list | inserting MovementPathlist : {:?}",
                finished_path.clone()
            );
        }

        sprite_writer.send(PathSpriteEvent::spawn_move_path(finished_path.clone()));
        commands
            .insert_resource::<MovementPathList>(MovementPathList::new_from_path(finished_path));
    }
}

pub fn repath(
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    open_paths: ResMut<Paths>,
    mut path_list: Option<ResMut<MovementPathList>>,
    mut event_reader: EventReader<PathListEvent>,
    mut sprite_writer: EventWriter<PathSpriteEvent>,
) {
    // TODO: Look over this and test more to make sure it works as intended.
    // It should be fine to have find_map here, but there could be a possibility
    // that the event_reader would somehow get delayed, in which case this function
    // may have some unintended consequences.
    // Also see if there is a way to get rid of the clone
    let debug = false;
    if debug {
        println!("debug | repath | start repath");
    }
    if let Some(action) = event_reader.into_iter().find_map(|event| event.action) {
        if action == PathAction::Repath {
            if let Some(mut path_list) = path_list {
                let finished_path = find_path(open_paths, block_type_query, path_list.end).unwrap();
                // sprite_writer.send(SpriteAction::Despawn.into());
                path_list.add_focused(finished_path.clone());
                sprite_writer.send(PathSpriteEvent::spawn_move_path(finished_path));
            }
        }
    }
}

fn find_path(
    mut open_paths: ResMut<'_, Paths>,
    // mut event_writer: EventWriter<'_, MovementPathEvent>,
    block_type_query: Query<'_, '_, (&BlockType, &Transform), Without<PlayerComponent>>,
    end: Vec3,
) -> Option<MovementPath> {
    let debug = false;
    if debug {
        println!("debug | find_path | start find_path");
    }
    while open_paths.paths.iter().any(|node| node.open) {
        let closest = open_paths
            .paths
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
            return Some(MovementPath::new_active(queue));
        }
        let possible_paths =
            wall_collision_check(closest.pos, &block_type_query).open_nodes(closest, end);
        closest.open = false;

        open_paths.paths.extend_from_slice(&possible_paths);
    }
    None
}

pub fn move_path_system(
    mut player_query: Query<(&PlayerComponent, &mut PlayerAnimation, &mut Transform)>,
    move_que: Option<ResMut<MovementPath>>,
    mut movement: ResMut<Movement>,
    time: Res<Time>,
    mut event_writer: EventWriter<MovementPathEvent>,
) {
    let debug = false;
    if debug {
        println!("debug | move_path_system | start move_path_system");
    }
    let (player_stats, _player_animation, mut transform) = player_query.single_mut();
    if let Some(mut move_que) = move_que {
        if !movement.moving {
            if let Some((start, end)) = move_que.path.pop() {
                let delta = end - start;
                movement.set_target(
                    transform.translation.truncate(),
                    delta.truncate(),
                    player_stats.speed,
                );
            } else {
                event_writer.send(MovementPathEvent {
                    move_path: None,
                    action: Some(MovePathAction::Remove),
                });
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
}

pub fn move_event_system(
    mut commands: Commands,
    move_que: Option<Res<MovementPath>>,
    mut event_reader: EventReader<MovementPathEvent>,
    movement_mode: Res<MovementModeRes>,
) {
    let debug = false;
    if debug {
        println!("debug | move_event_system | start move_event_system");
    }
    if **movement_mode == MovementMode::TurnBasedMovement {
        if let Some(event) = event_reader.into_iter().next() {
            if let Some(action) = event.action {
                match action {
                    MovePathAction::Insert => {
                        commands
                            .insert_resource::<MovementPath>(event.move_path.to_owned().unwrap());
                    }
                    MovePathAction::Remove => {
                        if move_que.is_some() {
                            commands.remove_resource::<MovementPath>();
                            commands.remove_resource::<MovementPathList>();
                        }
                    } // MovePathAction::TakePath => todo!(),
                }
            }
        }
    }
}

#[derive(Event, Debug, Clone)]
pub struct PathListEvent {
    path_list: Option<MovementPathList>,
    action: Option<PathAction>,
}

impl From<PathAction> for PathListEvent {
    fn from(value: PathAction) -> Self {
        Self {
            path_list: None,
            action: Some(value),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PathAction {
    Remove,
    Repath,
    StartPath,
}

pub fn path_list_cleanup(
    mut commands: Commands,
    mut list_event_reader: EventReader<PathListEvent>,
    mut sprite_writer: EventWriter<PathSpriteEvent>,
) {
    for event in list_event_reader.into_iter() {
        if let Some(action) = event.action {
            if action == PathAction::Remove {
                commands.remove_resource::<MovementPathList>();
                sprite_writer.send(SpriteAction::Despawn.into());
            }
        }
    }
}
