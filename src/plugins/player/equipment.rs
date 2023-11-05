use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::equipment::weapon::WeaponName;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum WeaponSlotName {
    MainHand,
    OffHand,
    NaturalPrimary,
    NaturalSecondary,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct WeaponSlot {
    slot: WeaponSlotName,
    weapon_name: WeaponName,
    entity: Entity,
}
