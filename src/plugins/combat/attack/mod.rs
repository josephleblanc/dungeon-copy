use bevy::prelude::*;

use self::armor_class::sum_ac_modifiers;
use self::attack_roll::{sum_attack_modifier, AttackBonusEvent, AttackBonusSumEvent};
use self::critical_range::{sum_crit_range_mods, CritThreatModSumEvent};
use crate::plugins::item::equipment::weapon::EquippedWeapons;
use crate::plugins::player::attacks::IterativeAttack;
use crate::plugins::player::equipment::{WeaponSlot, WeaponSlotName};
use crate::resources::dice::Dice;
use crate::scenes::SceneState;
use crate::{
    components::attack_bonus::BaseAttackBonus,
    plugins::{
        interact::{InteractingPos, InteractingType},
        player::control::ActionPriority,
    },
};

pub mod ac_modifier;
pub mod armor_class;
pub mod attack_roll;
pub mod attack_roll_modifier;
pub mod crit_multiplier;
pub mod crit_multiplier_modifier;
pub mod critical_range;
pub mod critical_range_modifier;
pub mod damage;
pub mod damage_modifier;

use crate::plugins::combat::attack::armor_class::{ACBonusEvent, ACBonusSumEvent};

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct AttackRollModifier;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct SumRollModifier;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct AttackRollComplete;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartAttack>()
            .add_event::<AttackDataEvent>()
            // AC-related Events
            .add_event::<armor_class::ACBonusEvent>()
            .add_event::<armor_class::ACBonusSumEvent>()
            .add_event::<ac_modifier::ACModEvent>()
            // Attack roll related events
            .add_event::<attack_roll::AttackBonusEvent>()
            .add_event::<attack_roll::AttackBonusSumEvent>()
            .add_event::<attack_roll_modifier::AttackModEvent>()
            // Critical Threat related events
            .add_event::<critical_range_modifier::CritThreatModEvent>()
            .add_event::<critical_range::CritThreatModSumEvent>()
            // Attack outcome and associated data
            .add_event::<CompleteAttackEvent>();

        app.configure_set(Update, AttackRollModifier.after(start_attack));
        app.configure_set(Update, SumRollModifier.after(AttackRollModifier));
        app.configure_set(Update, AttackRollComplete.after(SumRollModifier));

        app.add_systems(
            Update,
            (
                check_attack_conditions,
                start_attack.after(check_attack_conditions),
            )
                .run_if(in_state(SceneState::InGameClassicMode)),
        );

        app.add_systems(
            Update,
            (
                attack_roll_modifier::base_attack_bonus,
                attack_roll_modifier::add_strength,
                attack_roll_modifier::add_weapon_focus,
                critical_range_modifier::base,
                critical_range_modifier::improved_critical,
                ac_modifier::add_dexterity,
            )
                .in_set(AttackRollModifier),
        );

        app.add_systems(
            Update,
            (sum_ac_modifiers, sum_attack_modifier, sum_crit_range_mods).in_set(SumRollModifier),
        );

        app.add_systems(Update, evaluate_complete_attack.in_set(AttackRollComplete));
    }
}

#[derive(Event)]
/// This event is sent by `check_attack_conditions` and listened to by `start_attack`. This
/// singleton event is the signal that all of the conditions have been met for the attack roll and
/// damage systems to run without causing a `panic`.
pub struct StartAttack;

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

#[derive(Copy, Clone, Deref, DerefMut, Event, Debug)]
/// `AttackDataEvent` is used to send the entities which are used in the attack to the
/// attack/crit/damage systems.
pub struct AttackDataEvent(AttackData);

/// This is where the attack roll process begins. Once all of the conditions have been met this
/// function will send an event that lets `start_attack` begin.
/// Any conditions that must be met for an attack to begin should be put here, along with anything
/// that could cause a `panic` in the systems which compose the process of attacking and applying
/// damage.
pub fn check_attack_conditions(
    interacting_pos: Res<InteractingPos>,
    mut attack_event_writer: EventWriter<StartAttack>,
    button: Res<Input<MouseButton>>,

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
    if interacting_pos.interacting_type == InteractingType::Enemy
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

/// start_attack runs when the conditions for an attack have been met in `check_attack_conditions`.
/// Then it sends two events, one to start the systems calculating the total of the attack modifiers,
/// another to the systems calculating the ac of the defender.
/// There are multiple systems which check whether each modifier should be applied to ac and attack,
/// and then send an event listened to by `sum_attack_modifier`.
pub fn start_attack(
    mut start_attack_events: EventReader<StartAttack>,
    mut attack_event_writer: EventWriter<AttackBonusEvent>,
    mut ac_event_writer: EventWriter<ACBonusEvent>,
) {
    let debug = false;
    if !start_attack_events.is_empty() {
        start_attack_events.clear();

        if debug {
            println!("debug | attack::start_attack | start");
        }

        attack_event_writer.send(AttackBonusEvent);
        ac_event_writer.send(ACBonusEvent);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// AttackOutcome is the enum which describes the outcome of an attack roll, with modifiers
/// applied, against a valid target.
/// attack_roll --> AttackRollEvent --> Crit_threat_modifer::*
pub enum AttackOutcome {
    CritHit,
    Hit,
    Miss,
    CritMiss,
}

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
    // crit_multiplier: CritMultiplier,
    roll_raw: usize,
    roll_total: isize,
    defender_ac: isize,
    attack_data: AttackData,
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
    mut crit_range_mod_finished: EventReader<CritThreatModSumEvent>,
    mut complete_attack_writer: EventWriter<CompleteAttackEvent>,
    attacker_query: Query<&BaseAttackBonus>,
) {
    for (((attack_data, ac_mod), atk_mod), crit_range_mod) in attack_data_event
        .into_iter()
        .zip(ac_mod_finished.into_iter())
        .zip(atk_mod_finished.into_iter())
        .zip(crit_range_mod_finished.into_iter())
        .inspect(|(((attack_data, ac_mod), atk_mod), crit_range_mod)| {
            let ac_mod = ***attack_data == ac_mod.attack_data;
            let atk_mod = ***attack_data == atk_mod.attack_data;
            let crit_range_mod = ***attack_data == crit_range_mod.attack_data;
            println!(
                "inspecting: ac_mod {}, atk_mod {},crit_range_mod {}",
                ac_mod, atk_mod, crit_range_mod
            );
        })
        .filter(|(((attack_data, ac_mod), atk_mod), crit_range_mod)| {
            ***attack_data == ac_mod.attack_data
                && ***attack_data == atk_mod.attack_data
                && ***attack_data == crit_range_mod.attack_data
        })
    {
        println!("{:-<10}", "start evaluate_complete_attack");
        let attacker_bab = attacker_query.get(attack_data.attacker).unwrap();
        let mut rng = rand::thread_rng();
        let attack_roll_raw = Dice::D20.roll_once(&mut rng);
        let attack_modifier = atk_mod.total_attack_bonus + **attacker_bab;
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

        let complete_attack = CompleteAttack {
            attack_modifier,
            outcome,
            crit_range_lower,
            roll_raw: attack_roll_raw,
            roll_total: attack_roll_total,
            defender_ac: total_defender_ac,
            attack_data: **attack_data,
        };
        complete_attack_writer.send(CompleteAttackEvent(complete_attack));
        debug_complete_attack(
            attack_roll_raw,
            attacker_bab,
            atk_mod,
            ac_mod,
            attack_roll_total,
            outcome,
            **attack_data,
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

fn debug_complete_attack(
    attack_roll: usize,
    attacker_bab: &BaseAttackBonus,
    atk_event: &AttackBonusSumEvent,
    ac_event: &ACBonusSumEvent,
    attack_roll_total: isize,
    attack_outcome: AttackOutcome,
    attack_data: AttackData,
) {
    println!("      |                     | D20 roll: {}", attack_roll);
    println!("      |                     | BAB: {}", **attacker_bab);
    println!(
        "      |                     | summed attack modifiers: {}",
        atk_event.total_attack_bonus
    );
    println!(
        "      |                     | attack bonus total: {}",
        atk_event.total_attack_bonus + **attacker_bab
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
}
