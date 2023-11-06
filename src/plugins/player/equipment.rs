use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::equipment::weapon::WeaponName;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeaponSlotName {
    TwoHanded,
    MainHand,
    OffHand,
    NaturalPrimary,
    NaturalSecondary,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WeaponSlot {
    pub slot: WeaponSlotName,
    pub entity: Entity,
}
