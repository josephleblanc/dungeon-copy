use bevy::prelude::*;

use crate::components::creature::Creature;

use self::aoo_round_modifier::{AOORoundMod, AOORoundModEvent};

use super::bonus::BonusType;

pub mod aoo_round_modifier;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct AOORoundPlugin;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct ModSet;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct SumSet;

impl Plugin for AOORoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AOORoundStart>()
            .add_event::<AOORoundModEvent>()
            .add_event::<AOORoundSumEvent>()
            .configure_sets(
                Update,
                (
                    ModSet.run_if(on_event::<AOORoundStart>()),
                    SumSet.after(ModSet).run_if(on_event::<AOORoundModEvent>()),
                ),
            )
            .add_systems(
                Update,
                (
                    (
                        aoo_round_modifier::CombatReflexes::add_bonus,
                        aoo_round_modifier::base,
                    )
                        .in_set(ModSet),
                    sum_aoo_round_modifier.in_set(SumSet),
                ),
            );
    }
}

pub fn sum_aoo_round_modifier(
    mut atk_mod_events: EventReader<AOORoundModEvent>,
    mut atk_mod_finished: EventWriter<AOORoundSumEvent>,
    query_creatures: Query<Entity, With<Creature>>,
) {
    let all_atk_mod_list: AOORoundModList =
        atk_mod_events.into_iter().map(|event| **event).collect();
    for entity in query_creatures.iter()
    // let entity_list = all_atk_mod_list.iter().map(|event| event.attacker).collect::<Vec<Entity>>();
    {
        let atk_mod_list: AOORoundModList = all_atk_mod_list
            .clone()
            .into_iter()
            .filter(|e| e.attacker == entity)
            .collect();
        if !atk_mod_list.is_empty() {
            // let attacker = atk_mod_list.verified_attacker().unwrap();
            let sum_event = AOORoundSumEvent {
                attacker: entity,
                aoo_per_round: atk_mod_list.sum_all(),
            };

            atk_mod_finished.send(sum_event);
        }
    }
}

#[derive(Copy, Clone, Debug, Event, Deref)]
pub struct AOORoundStart(Entity);

impl AOORoundStart {
    pub fn new(entity: Entity) -> Self {
        AOORoundStart(entity)
    }
}

#[derive(Copy, Clone, Event)]
pub struct AOORoundSumEvent {
    pub attacker: Entity,
    pub aoo_per_round: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AOOBonusSource {
    Base,
    Dexterity,
}

#[derive(Debug, Deref)]
pub struct AOORoundModList(Vec<AOORoundMod>);

impl AOORoundModList {
    fn new() -> AOORoundModList {
        AOORoundModList(Vec::new())
    }

    fn add(&mut self, elem: AOORoundMod) {
        self.0.push(elem);
    }

    fn sum_stackable(&self) -> isize {
        let debug = true;
        let mut total = 0;
        for bonus_type in BonusType::stackable() {
            total += self
                .iter()
                .filter(|atk_mod| atk_mod.bonus_type == bonus_type)
                .fold(0, |acc, x| acc + x.val);
            if debug {
                debug_sum_stackable(bonus_type, total);
            }
        }
        total
    }

    fn sum_non_stackable(&self) -> isize {
        let debug = true;
        let mut total = 0;
        for bonus_type in BonusType::non_stackable() {
            if let Some(highest_modifier) = self
                .iter()
                .filter(|atk_mod| atk_mod.bonus_type == bonus_type)
                .max_by(|x, y| x.val.cmp(&y.val))
            {
                total += highest_modifier.val;
                if debug {
                    debug_sum_non_stackable(bonus_type, total);
                }
            }
        }
        total
    }

    pub fn sum_all(&self) -> usize {
        0.max((self.sum_stackable() + self.sum_non_stackable()) as usize)
    }

    pub fn verified_attacker(&self) -> Result<Entity, &'static str> {
        if self.is_empty() {
            Err("Attempted to verify an empty list of AOORoundMods. \
                AOORoundModList must have at least one element")
        } else if self
            .iter()
            .any(|atk_mod| atk_mod.attacker != self[0].attacker)
        {
            Err("Mismatched data in AOORoundModList")
        } else {
            Ok(self[0].attacker)
        }
    }
}

fn debug_sum_non_stackable(bonus_type: BonusType, total: isize) {
    println!(
        "debug | attack_of_opportunity::sum_non_stackable | bonus type: {:?}, total: {}",
        bonus_type, total
    );
}

fn debug_sum_stackable(bonus_type: BonusType, total: isize) {
    println!(
        "debug | attack_of_opportunity::sum_stackable | bonus type: {:?}, total: {}",
        bonus_type, total
    );
}

impl FromIterator<AOORoundMod> for AOORoundModList {
    fn from_iter<I: IntoIterator<Item = AOORoundMod>>(iter: I) -> Self {
        let mut c = AOORoundModList::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
