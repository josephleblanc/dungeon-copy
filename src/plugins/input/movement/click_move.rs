use std::ops::Deref;
use std::ops::DerefMut;

use bevy::prelude::*;

use crate::plugins::game_ui::map::pathing::PathSpriteEvent;
use crate::plugins::game_ui::map::pathing::SpriteAction;
use crate::plugins::input::movement::move_event::MovePathAction;
use crate::plugins::input::movement::move_event::MovementPathEvent;
use crate::plugins::input::movement::path_list_event::{PathListAction, PathListEvent};
use crate::plugins::input::movement::path_move::MovementPath;
use crate::plugins::input::movement::Movement;
use crate::plugins::input::movement::MovementMode;
use crate::plugins::input::movement::MovementModeRes;
use crate::plugins::input::movement::PlayerComponent;
use crate::plugins::interact::InteractingPos;
use crate::plugins::player::collisions::wall_collision_check;
use crate::resources::dungeon::block_type::BlockType;

use super::map::MapGrid;

#[derive(Resource, Clone, Debug)]
pub struct MovementPathList {
    pub list: Vec<MovementPath>,
    pub focused: MovementPath,
    pub start: Vec3,
    pub end: Vec3,
    pub displayed: bool,
    pub active: bool,
    pub move_ready: bool,
}

impl MovementPathList {
    /// Creates a new MovementPathList from a MovementPath. By default the new
    /// MovementPathList will have a `shortest` and `focused` of clones of the
    /// new path.
    pub fn new_from_path(path: MovementPath) -> Self {
        let (_, end) = *path.path.first().unwrap();
        let (start, _) = *path.path.last().unwrap();
        MovementPathList {
            list: vec![path.clone()],
            focused: path.clone(),
            start,
            end,
            displayed: false,
            active: false,
            move_ready: false,
        }
    }

    /// Adds a new path to the list. Consumes by default, clone if needed.
    pub fn add_to_list(&mut self, path: MovementPath) {
        self.list.push(path);
    }

    pub fn set_focused(&mut self, mut new_focused: MovementPath) {
        self.focused = new_focused;
    }

    pub fn add_focused(&mut self) {
        if !self.list.as_slice().contains(&self.focused) {
            self.end = self.focused.end();
            self.list.push(self.focused.clone());
        }
    }

    pub fn to_event(&self, action: Option<PathListAction>) -> PathListEvent {
        PathListEvent::new(Some(self.list.clone()), action)
    }

    pub fn list_to_path(&self) -> MovementPath {
        let mut new_list: Vec<(Vec3, Vec3)> = self
            .list
            .clone()
            .into_iter()
            .rev()
            .flat_map(|move_path| move_path.path)
            .collect();
        new_list.dedup();

        MovementPath::new_inactive(new_list)
    }
}

#[derive(Clone, Debug)]
pub struct MoveNode {
    pub pos: Vec3,
    pub dist: f32,
    pub open: bool,
    pub path: Vec<Vec3>,
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

#[derive(Copy, Clone, Deref, DerefMut, Resource, Default)]
pub struct PathConditions(bool);

pub fn check_path_conditions(
    interacting_pos: Res<InteractingPos>,
    movement_mode: Res<MovementModeRes>,
    player_query: Query<(&PlayerComponent, &Transform)>,
    map_grid: Res<MapGrid>,
    movement: Res<Movement>,
    movement_path: Option<Res<MovementPath>>,
    mut path_ready: ResMut<PathConditions>,
) {
    let player_pos = player_query.get_single().unwrap().1.translation.truncate();
    let focus_pos = interacting_pos.pos;
    **path_ready = interacting_pos.active
        && **movement_mode == MovementMode::TurnBasedMovement
        && !movement.moving
        && player_pos != focus_pos
        && map_grid.positions.as_slice().contains(&player_pos)
        && (if let Some(move_path) = movement_path {
            move_path.is_traversing()
        } else {
            movement_path.is_none()
        })
}

pub fn handle_path(
    mut path_list: Option<ResMut<MovementPathList>>,
    button: Res<Input<MouseButton>>,
    mut list_event_writer: EventWriter<PathListEvent>,
    mut move_event_writer: EventWriter<MovementPathEvent>,

    interacting_pos: Res<InteractingPos>,
    path_ready: Res<PathConditions>,
) {
    let focus_pos = interacting_pos.pos;
    if **path_ready {
        if let Some(mut path_list) = path_list {
            if button.just_pressed(MouseButton::Left) {
                if focus_pos == path_list.focused.end().truncate() {
                    if !(path_list.end == path_list.focused.end()) || !path_list.active {
                        path_list.active = true;
                        path_list.add_focused();
                    } else {
                        let mut move_event: MovementPathEvent =
                            MovePathAction::InsertOrActivate.into();
                        let move_path = path_list.list_to_path();
                        println!("move_path end: {}", move_path.end());
                        println!("focused end: {}", path_list.focused.end());
                        move_event.set_move_path(move_path);
                        move_event_writer.send(move_event);
                    }
                }
            } else if button.just_pressed(MouseButton::Right) {
                list_event_writer.send(PathListAction::Remove.into());
                move_event_writer.send(MovePathAction::Remove.into());
            } else {
                if interacting_pos.is_changed() {
                    if path_list.active {
                        list_event_writer.send(PathListAction::AddPath.into());
                    } else {
                        list_event_writer.send(PathListAction::Remove.into());
                    }
                }
            }
        } else {
            list_event_writer.send(PathListAction::StartPath.into());
        }
    }
}

#[derive(Resource, Clone, Default)]
pub struct PathNodes {
    pub paths: Vec<MoveNode>,
    pub ignore: Vec<MoveNode>,
}

impl PathNodes {
    pub fn ignore_node(&mut self, node: MoveNode) {
        self.ignore.push(node);
    }
}

impl Deref for PathNodes {
    type Target = Vec<MoveNode>;
    fn deref(&self) -> &Self::Target {
        &self.paths
    }
}

impl DerefMut for PathNodes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.paths
    }
}

impl PathNodes {
    pub fn reset(&mut self) {
        self.paths = vec![];
    }
}

pub fn start_path_list(
    mut commands: Commands,
    player_query: Query<(&PlayerComponent, &Transform)>,
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    interacting_pos: Res<InteractingPos>,
    mut open_paths: ResMut<PathNodes>,
    mut sprite_writer: EventWriter<PathSpriteEvent>,
    mut event_reader: EventReader<PathListEvent>,
) {
    open_paths.reset();
    if event_reader
        .into_iter()
        .filter_map(|event| event.action)
        .any(|action| action == PathListAction::StartPath)
    {
        let (_player, transform) = player_query.single();

        let start = transform.translation;
        let end = Vec3::new(interacting_pos.pos.x, interacting_pos.pos.y, start.z);

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

        sprite_writer.send(PathSpriteEvent::spawn_move_path(finished_path.clone()));
        commands
            .insert_resource::<MovementPathList>(MovementPathList::new_from_path(finished_path));
    }
}

pub fn add_path(
    mut commands: Commands,
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    mut open_paths: ResMut<PathNodes>,
    mut path_list: Option<ResMut<MovementPathList>>,
    mut event_reader: EventReader<PathListEvent>,
    mut sprite_writer: EventWriter<PathSpriteEvent>,
    interacting_pos: Res<InteractingPos>,
) {
    // TODO: Look over this and test more to make sure it works as intended.
    // It should be fine to have fthind_map here, but there could be a possibility
    // that the event_reader would somehow get delayed, in which case this function
    // may have some unintended consequences.
    // Also see if there is a way to get rid of the clone

    open_paths.reset();
    let debug = true;
    if debug {
        println!("debug | add_path start");
    }
    if let Some(action) = event_reader.into_iter().find_map(|event| event.action) {
        if action == PathListAction::AddPath {
            if let Some(mut path_list) = path_list {
                let start = path_list.end;
                let end = Vec3::new(interacting_pos.pos.x, interacting_pos.pos.y, start.z);
                if debug {
                    println!("debug | add_path | start: {}, end: {}", start, end);
                }
                let start_node = MoveNode {
                    pos: start,
                    dist: start.distance(end),
                    open: true,
                    path: vec![start],
                };
                open_paths.paths = vec![start_node];
                let new_path = find_path(open_paths, block_type_query, end).unwrap();

                path_list.set_focused(new_path.clone());
                let mut total_path = path_list.clone().list_to_path();
                total_path.join(path_list.focused.clone());
                sprite_writer.send(PathSpriteEvent::spawn_move_path(total_path.clone()));
            }
        }
    }
}

fn find_path(
    mut open_paths: ResMut<'_, PathNodes>,
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
            return Some(MovementPath::new_inactive(queue));
        }
        let possible_paths =
            wall_collision_check(closest.pos, &block_type_query).open_nodes(closest, end);
        closest.open = false;

        open_paths.paths.extend_from_slice(&possible_paths);
    }
    None
}

pub fn path_list_cleanup(
    mut commands: Commands,
    mut list_event_reader: EventReader<PathListEvent>,
    mut sprite_writer: EventWriter<PathSpriteEvent>,
) {
    for event in list_event_reader.into_iter() {
        if let Some(action) = event.action {
            if action == PathListAction::Remove {
                commands.remove_resource::<MovementPathList>();
                sprite_writer.send(SpriteAction::Despawn.into());
            }
        }
    }
}
