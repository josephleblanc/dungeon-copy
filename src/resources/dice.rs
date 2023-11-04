use bevy::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialOrd, Ord, Eq, PartialEq)]
pub enum Dice {
    D2,
    D3,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl From<Dice> for Uniform<usize> {
    fn from(val: Dice) -> Self {
        use Dice::*;
        match val {
            D2 => Uniform::new_inclusive(1, 2),
            D3 => Uniform::new_inclusive(1, 3),
            D4 => Uniform::new_inclusive(1, 4),
            D6 => Uniform::new_inclusive(1, 6),
            D8 => Uniform::new_inclusive(1, 8),
            D10 => Uniform::new_inclusive(1, 10),
            D12 => Uniform::new_inclusive(1, 12),
            D20 => Uniform::new_inclusive(1, 20),
        }
    }
}

impl Dice {
    pub fn roll_once<R: Rng + ?Sized>(self, rng: &mut R) -> usize {
        let die_range: Uniform<usize> = self.into();
        let mut roll_die = rng.sample_iter(die_range);
        roll_die.next().unwrap()
    }

    pub fn roll_multiple<R: Rng + ?Sized, const N: usize>(self, rng: &mut R) -> [usize; N] {
        let mut rolls: [usize; N] = [1; N];

        let die_range: Uniform<usize> = self.into();
        let mut roll_die = rng.sample_iter(die_range);

        for roll in rolls.iter_mut() {
            *roll = roll_die.next().unwrap();
        }
        rolls
    }
}
