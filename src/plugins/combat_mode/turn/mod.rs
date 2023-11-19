use bevy::prelude::*;

use crate::plugins::combat::attack_of_opportunity::AOORoundSumEvent;

use self::action::TurnActions;

use super::{initiative::EndInitiative, TurnOrder};

pub mod action;

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                action::setup_turn
                    .run_if(resource_exists::<TurnOrder>().and_then(on_event::<EndInitiative>())),
                TurnActions::update_aoo_round.run_if(on_event::<AOORoundSumEvent>()),
            ),
        );
    }
}
