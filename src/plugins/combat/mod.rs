use bevy::prelude::*;

use self::attack::{crit_multiplier::CritMultiplier, AttackData, AttackOutcome, AttackPlugin};

pub mod attack;
pub mod attack_damage;
pub mod bonus;

pub struct CombatPlugin;

#[derive(Event, Copy, Clone, Deref)]
/// AttackRollEvent is the event sent out by `attack_roll`, and includes the outcome of an attack
/// against a valid target. This event is listened to by `start_damage`, which is the gatekeeper
/// for the systems which calculate the attack's damage.
pub struct CompleteAttackEvent(CompleteAttack);

#[derive(Copy, Clone)]
pub struct CompleteAttack {
    outcome: AttackOutcome,
    attack_modifier: isize,
    crit_range_lower: usize,
    crit_multiplier: CritMultiplier,
    roll_raw: usize,
    roll_total: isize,
    defender_ac: isize,
    attack_data: AttackData,
}

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            // Attack outcome and associated data, used by both AttackPlugin and AttackDamagePlugin
            .add_event::<CompleteAttackEvent>();
        app.add_plugins(AttackPlugin);
    }
}
