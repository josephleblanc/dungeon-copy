use bevy::prelude::*;
use std::slice::Iter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CritMultiplier {
    X2,
    X3,
    X4,
    X5,
    X6,
}

impl CritMultiplier {
    pub fn iterator() -> Iter<'static, CritMultiplier> {
        [
            CritMultiplier::X2,
            CritMultiplier::X3,
            CritMultiplier::X4,
            CritMultiplier::X5,
            CritMultiplier::X6,
        ]
        .iter()
    }
    pub fn size(self) -> usize {
        let (size, _) = CritMultiplier::iterator()
            .enumerate()
            .find(|(i, val)| self == **val)
            .unwrap();
        size as usize
    }
    pub fn add(self, other: Self) -> Self {
        let index = self.size() + other.size();
        let (_, crit) = CritMultiplier::iterator()
            .enumerate()
            .find(|(i, val)| index == *i)
            .unwrap();
        *crit
    }
    pub fn unchecked_add_one(self) -> Self {
        match self {
            CritMultiplier::X2 => CritMultiplier::X3,
            CritMultiplier::X3 => CritMultiplier::X4,
            CritMultiplier::X4 => CritMultiplier::X5,
            CritMultiplier::X5 => CritMultiplier::X6,
            CritMultiplier::X6 => CritMultiplier::X6,
        }
    }

    pub fn checked_add_one(self) -> Result<Self, &'static str> {
        match self {
            CritMultiplier::X2 => Ok(CritMultiplier::X3),
            CritMultiplier::X3 => Ok(CritMultiplier::X4),
            CritMultiplier::X4 => Ok(CritMultiplier::X5),
            CritMultiplier::X5 => Ok(CritMultiplier::X6),
            CritMultiplier::X6 => Err("Cannot increase critical past x6 you madman!!!"),
        }
    }

    pub fn increase_by(self, n: u8) -> Self {
        let mut out = self;
        for _ in 0..n {
            out = out.unchecked_add_one();
        }
        out
    }

    pub fn increase_with_limit(self, limit: Option<Self>) -> Self {
        let increased = self.unchecked_add_one();
        if let Some(crit_limit) = limit {
            if increased >= crit_limit {
                crit_limit
            } else {
                increased
            }
        } else {
            increased
        }
    }
}
