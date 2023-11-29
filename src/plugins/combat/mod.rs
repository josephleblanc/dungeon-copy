use bevy::prelude::*;

use crate::{
    resources::{dice::Dice, equipment::weapon::Weapon},
    scenes::SceneState,
};

use self::{
    attack::{
        armor_class::ACBonusSumEvent,
        attack_roll::AttackBonusSumEvent,
        crit_multiplier::{CritMultiplier, CritMultiplierSumEvent},
        critical_range::CritRangeModSumEvent,
        AttackOutcome, AttackPlugin, StartAttack,
    },
    attack_damage::{
        damage::AttackDamageSumEvent, damage_reduction::DRTotalEvent, AttackDamagePlugin,
    },
    attack_of_opportunity::AOORoundPlugin,
};

use super::{
    game_ui::action_bar::{ActionBarButton, SelectedAction},
    interact::{InteractingPos, InteractingType},
    item::equipment::weapon::EquippedWeapons,
    player::{
        attacks::IterativeAttack,
        control::ActionPriority,
        equipment::{WeaponSlot, WeaponSlotName},
    },
};

pub mod attack;
pub mod attack_damage;
pub mod attack_of_opportunity;
pub mod bonus;
pub mod damage;

pub struct CombatPlugin;

#[derive(Event, Copy, Clone, Deref)]
/// AttackRollEvent is the event sent out by `attack_roll`, and includes the outcome of an attack
/// against a valid target. This event is listened to by `start_damage`, which is the gatekeeper
/// for the systems which calculate the attack's damage.
pub struct CompleteAttackEvent(CompleteAttack);

#[derive(Copy, Clone, Deref, DerefMut, Event, Debug)]
/// `AttackDataEvent` is used to send the entities which are used in the attack to the
/// attack/crit/damage systems.
pub struct AttackDataEvent(pub AttackData);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// `AttackDataEvent` includes only those entities which are used in the attack, and
/// data which is only known at the time the attack is made, that is:
/// - `WeaponSlot`: Whether the attack is using main hand or off hand, or is two-handed, or is a
///     primary or secondary natural attack.
/// - `IterativeAttack`: If the character has more than +5 attack bonus and is using a weapon,
///     which of the iterative attack bonuses to apply.
/// When the attack/crit/damage systems need to know, e.g., what weapon type is used in the attack,
/// they can query the entity to find the relevent components, if they exist.
pub struct AttackData {
    pub weapon_slot: WeaponSlot,
    pub iterative_attack: IterativeAttack,
    pub attacker: Entity,
    pub defender: Entity,
}

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

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct AttackModifier;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct SumModifier;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct DebugSet;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            // Attack outcome and associated data, used by both AttackPlugin and AttackDamagePlugin
            .add_event::<CompleteAttackEvent>()
            .add_event::<AttackDataEvent>();

        app.add_plugins((AttackPlugin, AttackDamagePlugin, AOORoundPlugin));

        app.add_systems(
            Update,
            check_attack_conditions
                .run_if(in_state(SceneState::InGameClassicMode))
                .run_if(resource_exists_and_equals(SelectedAction(
                    ActionBarButton::Attack,
                ))),
        );

        app.configure_set(Update, AttackModifier.after(check_attack_conditions));
        app.configure_set(Update, SumModifier.after(AttackModifier));
        app.configure_set(Update, DebugSet.after(SumModifier));

        app.add_systems(Update, evaluate_complete_attack.after(SumModifier));
    }
}

/// This is where the attack roll process begins. Once all of the conditions have been met this
/// function will send an event that lets `start_attack` begin.
/// Any conditions that must be met for an attack to begin should be put here, along with anything
/// that could cause a `panic` in the systems which compose the process of attacking and applying
/// damage.
pub fn check_attack_conditions(
    interacting_pos: Res<InteractingPos>,
    mut attack_event_writer: EventWriter<StartAttack>,
    button: Res<Input<MouseButton>>,
    selected_action: Res<SelectedAction>,

    // TODO: Move the below arguments into the system which prompts the attack, once it has been
    // created.
    attacker_query: Query<(Entity, &EquippedWeapons), With<ActionPriority>>,
    mut attack_data_writer: EventWriter<AttackDataEvent>,
) {
    let debug = false;
    if debug && button.just_pressed(MouseButton::Left) {
        println!(
            "debug | check_attack_conditions | \
        interacting_pos.entity.is_some(): {}",
            interacting_pos.entity.is_some()
        );
    }
    if **selected_action == ActionBarButton::Attack
        && interacting_pos.interacting_type == InteractingType::Enemy
        && interacting_pos.entity.is_some()
        && button.just_pressed(MouseButton::Left)
    {
        // TODO: Check if target is in range
        attack_event_writer.send(StartAttack);

        // TODO: Only for testing, change values when moving to another system that will prompt an
        // attack.
        let (attacker_entity, equipped_weapon_entity) = attacker_query.get_single().unwrap();
        let start_attack_data = AttackData {
            weapon_slot: WeaponSlot {
                // The enum in `slot` should be be supplied by the system which prompts the attack.
                slot: WeaponSlotName::MainHand,
                entity: equipped_weapon_entity.main_hand,
            },
            // The enum in `iterative_attack` should be supplied by the system which prompts the
            // attack.
            iterative_attack: IterativeAttack::First,
            attacker: attacker_entity,
            defender: interacting_pos.entity.unwrap(),
        };
        attack_data_writer.send(AttackDataEvent(start_attack_data));
    }
}

/// `complete_attack` is the system which takes the sums of the attack bonus, critical threat, and
/// critical multiplier modifiers, along with defender ac modifiers, rolls the d20, and evaluates
/// the outcome of the attack.
/// If the attack is a `Hit` or `Crit` the `damage::start_damage` system will start calculating the
/// damage of the attack.
pub fn evaluate_complete_attack(
    mut attack_data_event: EventReader<AttackDataEvent>,
    mut ac_mod_finished: EventReader<ACBonusSumEvent>,
    mut atk_mod_finished: EventReader<AttackBonusSumEvent>,
    mut crit_range_mod_finished: EventReader<CritRangeModSumEvent>,
    mut crit_multiplier_mod_finished: EventReader<CritMultiplierSumEvent>,
    mut damage_finished: EventReader<AttackDamageSumEvent>,
    mut dr_total_reader: EventReader<DRTotalEvent>,
    mut complete_attack_writer: EventWriter<CompleteAttackEvent>,
    weapon_query: Query<&Weapon>,
) {
    for (
        (((((attack_data, ac_mod), atk_mod), crit_range_mod), crit_multiplier), damage),
        dr_total,
    ) in attack_data_event
        .into_iter()
        .zip(ac_mod_finished.into_iter())
        .zip(atk_mod_finished.into_iter())
        .zip(crit_range_mod_finished.into_iter())
        .zip(crit_multiplier_mod_finished.into_iter())
        .zip(damage_finished.into_iter())
        .zip(dr_total_reader.into_iter())
        .filter(|((((((data, ac), atk), crit_r), crit_m), dmg), dr)| {
            ***data == ac.attack_data
                && ***data == atk.attack_data
                && ***data == crit_r.attack_data
                && ***data == crit_m.attack_data
                && ***data == dmg.attack_data
                && ***data == dr.attack_data
        })
    {
        println!("{:-<10}", "start evaluate_complete_attack");
        let mut rng = rand::thread_rng();
        let attack_roll_raw = Dice::D20.roll_once(&mut rng);
        let attack_modifier = atk_mod.total_attack_bonus;
        let total_defender_ac = 10 + ac_mod.total_ac_bonus;
        let crit_range_lower = crit_range_mod.lower_crit();

        let attack_roll_total: isize = attack_roll_raw as isize + attack_modifier;
        let outcome = if attack_roll_raw == 20
            || (total_defender_ac <= attack_roll_total
                && attack_roll_raw >= crit_range_mod.lower_crit())
        {
            AttackOutcome::CritHit
        } else if attack_roll_raw == 1 {
            AttackOutcome::CritMiss
        } else if total_defender_ac <= attack_roll_total {
            AttackOutcome::Hit
        } else {
            AttackOutcome::Miss
        };
        let mut total_damage: Option<usize> = None;
        println!("crit_multiplier.size(): {}", crit_multiplier.size());
        println!("damage.weapon_damage: {}", damage.weapon_damage);
        println!("damage.multiply_on_crit: {}", damage.multiply_on_crit);
        println!("damage.no_multiply_on_crit: {}", damage.no_multiply_on_crit);
        println!("damage.only_on_crit: {}", damage.only_on_crit);
        if outcome == AttackOutcome::Hit || outcome == AttackOutcome::CritHit {
            // TODO: change crit to roll weapon damage and any other rolled damage that is
            // multiplied on a critical hit.
            let crit_damage = if outcome == AttackOutcome::CritHit {
                crit_multiplier.size() as isize * (damage.multiply_on_crit + damage.weapon_damage)
                    + damage.only_on_crit
            } else {
                0
            };

            total_damage = Some(
                (damage.no_multiply_on_crit + damage.weapon_damage).max(0) as usize
                    + crit_damage.max(0) as usize,
            );

            // Get the best DR matchup this weapon can deal, and apply DR if any is applicable.
            // If there is no DR applicable, best_dr will be None
            let weapon = weapon_query.get(attack_data.weapon_slot.entity).unwrap();
            let best_dr = dr_total.min_vs_weapon(weapon.weapon_damage_types);
            if let Some(lowest_dr) = best_dr {
                total_damage = Some(total_damage.unwrap().saturating_sub(lowest_dr));
            }
        }

        let complete_attack = CompleteAttack {
            attack_modifier,
            outcome,
            crit_range_lower,
            roll_raw: attack_roll_raw,
            roll_total: attack_roll_total,
            defender_ac: total_defender_ac,
            attack_data: **attack_data,
            crit_multiplier: crit_multiplier.val,
        };
        complete_attack_writer.send(CompleteAttackEvent(complete_attack));
        debug_complete_attack(
            attack_roll_raw,
            atk_mod,
            ac_mod,
            attack_roll_total,
            outcome,
            **attack_data,
            crit_multiplier.val,
            total_damage,
        );
    }
}

pub fn debug_complete_attack_event(mut attack_roll_event_reader: EventReader<CompleteAttackEvent>) {
    let debug = false;
    if debug {
        for event in attack_roll_event_reader.iter() {
            println!("attack roll outcome: {:?}", event.outcome);
        }
    }
}

fn debug_evaluate_attack_complete(
    mut attack_data_event: EventReader<'_, '_, AttackDataEvent>,
    mut ac_mod_finished: EventReader<'_, '_, ACBonusSumEvent>,
    mut atk_mod_finished: EventReader<'_, '_, AttackBonusSumEvent>,
    mut crit_range_mod_finished: EventReader<'_, '_, CritRangeModSumEvent>,
    mut crit_multiplier_mod_finished: EventReader<'_, '_, CritMultiplierSumEvent>,
) {
    for ((((attack_data, ac_mod), atk_mod), crit_range_mod), crit_multiplier) in attack_data_event
        .into_iter()
        .zip(ac_mod_finished.into_iter())
        .zip(atk_mod_finished.into_iter())
        .zip(crit_range_mod_finished.into_iter())
        .zip(crit_multiplier_mod_finished.into_iter())
        .inspect(|((((data, ac), atk), crit_r), crit_m)| {
            let ac = ***data == ac.attack_data;
            let atk = ***data == atk.attack_data;
            let crit_r = ***data == crit_r.attack_data;
            let crit_m = ***data == crit_m.attack_data;
            println!(
                "debug | evaluate_complete_attack | inspecting iterator: \
                \n\tcheck that attack_data is equal: ac {}, atk {}, crit_r {}, crit_m {}",
                ac, atk, crit_r, crit_m
            );
        })
    {}
}

fn debug_complete_attack(
    attack_roll: usize,
    atk_event: &AttackBonusSumEvent,
    ac_event: &ACBonusSumEvent,
    attack_roll_total: isize,
    attack_outcome: AttackOutcome,
    attack_data: AttackData,
    crit_multiplier: CritMultiplier,
    total_damage: Option<usize>,
) {
    println!("      |                     | D20 roll: {}", attack_roll);
    println!(
        "      |                     | summed attack modifiers: {}",
        atk_event.total_attack_bonus
    );
    println!(
        "      |                     | attack bonus total: {}",
        atk_event.total_attack_bonus
    );
    println!(
        "      |                     | defender AC bonus: {}",
        ac_event.total_ac_bonus
    );
    println!(
        "      |                     | defender AC total: {}",
        10 + ac_event.total_ac_bonus
    );
    println!(
        "      |                     | total attack roll with bonuses: {}",
        attack_roll_total
    );
    println!(
        "      |                     | attack outcome: {:?}",
        attack_outcome
    );
    println!(
        "      |                     | attack_data: {:?}",
        attack_data
    );
    println!(
        "      |                     | crit_multiplier: {:?}",
        crit_multiplier
    );
    println!(
        "      |                     | total_damage: {:?}",
        total_damage
    );
}
