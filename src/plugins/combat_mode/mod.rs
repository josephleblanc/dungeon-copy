use bevy::{prelude::*, utils::hashbrown::HashMap};

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
            .init_resource::<TurnOrder>()
            .add_systems(Update, start_initiative.in_set(StartSet))
            .add_systems(
                Update,
                (initiative_modifier::base_initiative).in_set(ModSet),
            )
            .add_systems(Update, sum_initiative_modifiers.in_set(SumSet));
    }
}

#[derive(Clone, Deref, DerefMut, Resource, Default)]
pub struct TurnOrder(HashMap<Entity, SummedInitiative>);

#[derive(Clone, Deref, DerefMut, Default)]
pub struct SummedInitiative {
    val: Initiative,
    #[deref]
    mods: Vec<InitiativeMod>,
}

impl SummedInitiative {
    fn add(&mut self, elem: InitiativeMod) {
        self.push(elem);
    }

    pub fn from(initiative_mod: InitiativeMod) -> Self {
        let mut summed = Self::new();
        summed.val = Initiative::from_isize(initiative_mod.val);
        summed.mods.push(initiative_mod);

        summed
    }
    pub fn new() -> Self {
        SummedInitiative {
            val: Initiative::from_isize(0),
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
                .fold(0, |acc, x| acc + x.val);
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
                .max_by(|x, y| x.val.cmp(&y.val))
            {
                total += highest_modifier.val;
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

impl FromIterator<InitiativeMod> for SummedInitiative {
    fn from_iter<I: IntoIterator<Item = InitiativeMod>>(iter: I) -> Self {
        let mut c = SummedInitiative::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
