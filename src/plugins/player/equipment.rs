use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::equipment::weapon::WeaponName;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum WeaponSlotName {
    TwoHanded,
    MainHand,
    OffHand,
    NaturalPrimary,
    NaturalSecondary,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct WeaponSlot {
    pub slot: WeaponSlotName,
    pub entity: Entity,
}
