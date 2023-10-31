use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy, Deref, DerefMut, Serialize, Deserialize)]
pub struct HitPoints(isize);
