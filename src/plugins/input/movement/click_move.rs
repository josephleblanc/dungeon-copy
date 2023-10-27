use std::ops::Deref;
use std::ops::DerefMut;

use bevy::prelude::*;

use crate::plugins::game_ui::map::pathing::PathSpriteEvent;
use crate::plugins::game_ui::map::pathing::SpriteAction;
use crate::plugins::input::movement::move_event::MovePathAction;
use crate::plugins::input::movement::move_event::MovementPathEvent;
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
            active: true,
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
    path_list: Option<Res<MovementPathList>>,
    button: Res<Input<MouseButton>>,
    mut list_event_writer: EventWriter<PathListEvent>,
    mut move_event_writer: EventWriter<MovementPathEvent>,

    interacting_pos: Res<InteractingPos>,
    player_query: Query<(&PlayerComponent, &Transform)>,
    map_grid: Res<MapGrid>,
    path_ready: Res<PathConditions>,
) {
    let debug = false;
    // if debug {
    //     println!("debug | handle_path | start handle_path");
    // }

    let player_pos = player_query.get_single().unwrap().1.translation.truncate();
    let focus_pos = interacting_pos.pos;
    if **path_ready {
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
                println!(
                    "focus_pos: {}, path_list.end.truncate(): {}",
                    focus_pos,
                    path_list.end.truncate()
                );
                if focus_pos == path_list.end.truncate() {
                    move_event_writer.send(
                        path_list
                            .focused
                            .clone()
                            .to_event(MovePathAction::InsertOrActivate),
                    )
                }
            } else if button.just_pressed(MouseButton::Right) {
                list_event_writer.send(PathAction::Remove.into());
                move_event_writer.send(MovePathAction::Remove.into());
            } else {
                if interacting_pos.is_changed() {
                    if debug {
                        println!("interacting_pos changed");
                    }
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
    let debug = false;
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
    open_paths: ResMut<PathNodes>,
    mut path_list: Option<ResMut<MovementPathList>>,
    mut event_reader: EventReader<PathListEvent>,
    mut sprite_writer: EventWriter<PathSpriteEvent>,
) {
    // TODO: Look over this and test more to make sure it works as intended.
    // It should be fine to have fthind_map here, but there could be a possibility
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
