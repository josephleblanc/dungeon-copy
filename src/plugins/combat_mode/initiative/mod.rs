use bevy::prelude::*;

use crate::components::creature::Creature;

use self::initiative_modifier::InitiativeModEvent;

use super::{turn::action::CurrentTurn, InitiativeDetails, InitiativeMap, TurnOrder};

pub mod initiative_modifier;

#[derive(Event, Clone, Copy, Deref, DerefMut)]
pub struct StartInitiative(Entity);

impl StartInitiative {
    pub fn from(creature: Entity) -> Self {
        Self(creature)
    }
}

#[derive(Event, Clone, Copy)]
pub struct EndInitiative;

#[derive(Debug, Clone, Copy, Deref, DerefMut, Default)]
pub struct Initiative(isize);

impl Initiative {
    pub fn from_isize(other: isize) -> Self {
        Initiative(other)
    }
}

pub fn start_initiative(
    query_creatures: Query<Entity, With<Creature>>,
    mut event_writer: EventWriter<StartInitiative>,
    mut commands: Commands,
) {
    let debug = true;
    if debug {
        println!("debug | initiative::start_initiative | start");
    }
    for creature in query_creatures.iter() {
        if debug {
            println!("debug | initiative::start_initiative | start");
        }
        event_writer.send(StartInitiative::from(creature));
    }
    commands.init_resource::<InitiativeMap>();
    commands.init_resource::<TurnOrder>();
}

pub fn sum_initiative_modifiers(
    mut event_reader: EventReader<InitiativeModEvent>,
    mut end_initiative: EventWriter<EndInitiative>,
    mut initiative_map: ResMut<InitiativeMap>,
    mut turn_order: ResMut<TurnOrder>,
) {
    let debug = true;
    for event in event_reader.into_iter() {
        initiative_map
            .entry(event.entity)
            .and_modify(|e| e.push(**event))
            .or_insert(InitiativeDetails::from(**event));
    }

    for (entity, summed_initiative) in initiative_map.iter_mut() {
        summed_initiative.bonus = Initiative::from_isize(summed_initiative.sum_all());
        if debug {
            debug_sum_initiative_modifers(entity, summed_initiative);
        }
    }

    let mut rng = rand::thread_rng();
    *turn_order = TurnOrder::from_vec(initiative_map.generate_turn_order(&mut rng));

    end_initiative.send(EndInitiative);
}

fn debug_sum_initiative_modifers(entity: &Entity, summed_initiative: &mut InitiativeDetails) {
    println!(
        "entity: {:?} has initiative modifier: {:?}",
        entity, summed_initiative.bonus
    );
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<InitiativeMap>();
    commands.remove_resource::<TurnOrder>();
}
