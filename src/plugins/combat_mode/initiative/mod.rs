use bevy::prelude::*;

use crate::components::creature::Creature;

use self::initiative_modifier::InitiativeModEvent;

use super::{SummedInitiative, TurnOrder};

pub mod initiative_modifier;

#[derive(Event, Clone, Copy, Deref, DerefMut)]
pub struct StartInitiative(Entity);

impl StartInitiative {
    pub fn from(creature: Entity) -> Self {
        Self(creature)
    }
}

// #[derive(Event, Clone, Copy, Deref, DerefMut)]
// pub struct EndInitiative(Entity);
//
// impl EndInitiative {
//     pub fn from(creature: Entity) -> Self {
//         Self(creature)
//     }
// }

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
) {
    for creature in query_creatures.iter() {
        event_writer.send(StartInitiative::from(creature));
    }
}

pub fn sum_initiative_modifiers(
    mut event_reader: EventReader<InitiativeModEvent>,
    mut initiative_res: ResMut<TurnOrder>,
    // mut event_writer: EventWriter<EndInitiative>,
) {
    let debug = true;
    for event in event_reader.into_iter() {
        initiative_res
            .entry(event.entity)
            .and_modify(|e| e.push(**event))
            .or_insert(SummedInitiative::from(**event));
    }

    for (entity, summed_initiative) in initiative_res.iter_mut() {
        summed_initiative.val = Initiative::from_isize(summed_initiative.sum_all());
        if debug {
            println!(
                "entity: {:?} has initiative modifier: {:?}",
                entity, summed_initiative.val
            );
        }
    }
}
