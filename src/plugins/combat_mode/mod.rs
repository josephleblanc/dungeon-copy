use bevy::{prelude::*, utils::hashbrown::HashMap};
use rand::Rng;

use crate::resources::dice::Dice;

use self::{
    initiative::{
        initiative_modifier::{InitiativeMod, InitiativeModEvent},
        *,
    },
    state::CombatMode,
};

use super::{combat::bonus::BonusType, game_ui::combat_mode::CombatModeRes};

pub mod initiative;
pub mod state;

pub struct CombatModePlugin;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct StartSet;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct ModSet;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct SumSet;

impl Plugin for CombatModePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartInitiative>()
            .add_event::<InitiativeModEvent>()
            .configure_sets(
                Update,
                (
                    StartSet.run_if(
                        resource_exists_and_changed::<CombatModeRes>()
                            .and_then(resource_equals(CombatModeRes(CombatMode::InCombat))),
                    ),
                    ModSet.after(StartSet).run_if(
                        resource_exists_and_changed::<CombatModeRes>()
                            .and_then(resource_equals(CombatModeRes(CombatMode::InCombat))),
                    ),
                    SumSet.after(ModSet).run_if(
                        resource_exists_and_changed::<CombatModeRes>()
                            .and_then(resource_equals(CombatModeRes(CombatMode::InCombat))),
                    ),
                ),
            )
            // .init_resource::<InitiativeMap>()
            .add_systems(
                Update,
                (start_initiative, apply_deferred).chain().in_set(StartSet),
            )
            .add_systems(
                Update,
                (initiative_modifier::base_initiative).in_set(ModSet),
            )
            .add_systems(Update, sum_initiative_modifiers.in_set(SumSet));

        app.add_systems(
            Update,
            cleanup.run_if(
                resource_exists_and_changed::<CombatModeRes>()
                    .and_then(resource_equals(CombatModeRes(CombatMode::OutOfCombat))),
            ),
        );
    }
}

#[derive(Clone, Deref, DerefMut, Resource, Default)]
pub struct InitiativeMap(HashMap<Entity, InitiativeDetails>);

impl InitiativeMap {
    pub fn generate_turn_order<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Vec<(Entity, usize)> {
        let debug = true;
        let d20 = Dice::D20;
        self.iter_mut().for_each(|(_entity, init_details)| {
            init_details.total = Some(d20.roll_once(rng) as isize + *init_details.bonus);
        });
        let mut count = 1_usize;
        while self.iter().any(|(_k, v)| v.turn_index.is_none()) {
            let highest_entity = *self
                .iter()
                .filter(|(_k, v)| v.turn_index.is_none())
                .max_by(|x, y| x.1.total.unwrap().cmp(&y.1.total.unwrap()))
                .unwrap()
                .0;
            self.entry(highest_entity)
                .and_modify(|e| e.turn_index = Some(count));
            count += 1;
        }
        let mut turn_order: Vec<(Entity, usize)> = self
            .iter()
            .map(|(k, v)| (*k, v.turn_index.unwrap()))
            .collect();
        turn_order.sort_by(|x, y| x.1.cmp(&y.1));
        if debug {
            debug_generate_turn_order(&turn_order);
        }
        turn_order
    }
}

fn debug_generate_turn_order(turn_order: &Vec<(Entity, usize)>) {
    println!(
        "debug | InitiativeMap::generate_turn_order | turn_order pretty {:#?}",
        turn_order
    );
}

#[derive(Clone, Deref, DerefMut, Resource, Default)]
pub struct TurnOrder(Vec<(Entity, usize)>);

impl TurnOrder {
    pub fn from_vec(vec: Vec<(Entity, usize)>) -> Self {
        Self(vec)
    }
}

#[derive(Clone, Deref, DerefMut, Default)]
pub struct InitiativeDetails {
    bonus: Initiative,
    turn_index: Option<usize>,
    // A roll plus the bonus from the Initiative struct above.
    total: Option<isize>,
    #[deref]
    mods: Vec<InitiativeMod>,
}

impl InitiativeDetails {
    fn add(&mut self, elem: InitiativeMod) {
        self.push(elem);
    }

    pub fn from(initiative_mod: InitiativeMod) -> Self {
        let mut summed = Self::new();
        summed.bonus = Initiative::from_isize(initiative_mod.bonus);
        summed.mods.push(initiative_mod);

        summed
    }
    pub fn new() -> Self {
        InitiativeDetails {
            bonus: Initiative::from_isize(0),
            turn_index: None,
            total: None,
            mods: Vec::new(),
        }
    }

    fn sum_stackable(&self) -> isize {
        let debug = false;
        let mut total = 0;
        for bonus_type in BonusType::stackable() {
            total += self
                .iter()
                .filter(|atk_mod| atk_mod.bonus_type == bonus_type)
                .fold(0, |acc, x| acc + x.bonus);
            // if debug {
            //     debug_sum_stackable(bonus_type, total);
            // }
        }
        total
    }

    fn sum_non_stackable(&self) -> isize {
        let debug = false;
        let mut total = 0;
        for bonus_type in BonusType::non_stackable() {
            if let Some(highest_modifier) = self
                .iter()
                .filter(|atk_mod| atk_mod.bonus_type == bonus_type)
                .max_by(|x, y| x.bonus.cmp(&y.bonus))
            {
                total += highest_modifier.bonus;
                // if debug {
                //     debug_sum_non_stackable(bonus_type, total);
                // }
            }
        }
        total
    }

    pub fn sum_all(&self) -> isize {
        self.sum_stackable() + self.sum_non_stackable()
    }
}

impl FromIterator<InitiativeMod> for InitiativeDetails {
    fn from_iter<I: IntoIterator<Item = InitiativeMod>>(iter: I) -> Self {
        let mut c = InitiativeDetails::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
