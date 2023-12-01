use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeaponSlotName {
    TwoHanded,
    MainHand,
    OffHand,
    NaturalOnly,
    NaturalPrimary,
    NaturalSecondary,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WeaponSlot {
    pub slot: WeaponSlotName,
    pub entity: Entity,
}
