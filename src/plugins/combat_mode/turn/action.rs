#![allow(dead_code)]

use bevy::prelude::*;

use crate::plugins::{
    combat::attack_of_opportunity::{AOORoundStart, AOORoundSumEvent},
    combat_mode::{initiative::EndInitiative, TurnOrder},
};

#[derive(Resource, Copy, Clone)]
/// A resource with the Entity whose turn it currently is, along with the index of the entity in
/// the turn order tracker resource, TurnOrder.
pub struct CurrentTurn {
    pub entity: Entity,
    turn_index: usize,
}

impl CurrentTurn {
    pub fn new(entity: Entity, turn_index: usize) -> Self {
        Self { entity, turn_index }
    }
}

#[derive(Component, Copy, Clone)]
pub struct TurnActions {
    pub move_action: bool,
    pub standard_action: bool,
    pub immediate_action: bool,
    // Attacks of Oppertunity per round.
    pub aoo_round: usize,
}

impl TurnActions {
    pub fn new() -> Self {
        Self {
            move_action: false,
            standard_action: false,
            immediate_action: false,
            aoo_round: 0,
        }
    }

    pub fn update_aoo_round(
        mut aoo_events: EventReader<AOORoundSumEvent>,
        mut query: Query<&mut TurnActions>,
    ) {
        let debug = true;
        for event in aoo_events.into_iter() {
            if let Ok(mut turn_actions) = query.get_mut(event.attacker) {
                turn_actions.aoo_round = event.aoo_per_round;
                if debug {
                    println!("debug | action::TurnActions::aoo_events | entity {:?} set turn_actions.aoo_round = {}",
                event.attacker, turn_actions.aoo_round);
                }
            }
        }
    }
}

pub fn setup_turn(
    turn_order: Res<TurnOrder>,
    mut commands: Commands,
    mut end_initiative: EventReader<EndInitiative>,
    mut aoo_round_start: EventWriter<AOORoundStart>,
) {
    let debug = true;
    if !end_initiative.is_empty() {
        if debug {
            println!("debug | action::setup_turn | EndInitiative event received");
        }
        end_initiative.clear();

        let (first_turn_entity, turn_index) = turn_order.first().unwrap();
        commands.insert_resource(CurrentTurn::new(*first_turn_entity, *turn_index));

        for (entity, _turn_index) in turn_order.iter() {
            if debug {
                println!(
                    "debug | action::setup_turn | inserting turn actions for {:?}",
                    first_turn_entity
                );
            }
            commands.entity(*entity).insert(TurnActions::new());
            aoo_round_start.send(AOORoundStart::new(*entity));
        }
    }
}

pub fn reset_turn_actions() {}
