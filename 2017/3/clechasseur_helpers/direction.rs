use std::ops::Neg;

use num::{one, zero, One, Zero};
use strum_macros::FromRepr;

use crate::helpers::pt::Pt;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromRepr)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        Self::from_repr((*self as u8).wrapping_sub(1)).unwrap_or(Direction::Up)
    }

    pub fn turn_right(&self) -> Self {
        Self::from_repr((*self as u8) + 1).unwrap_or(Direction::Right)
    }

    pub fn displacement<T>(&self) -> Pt<T>
    where
        T: Zero + One + Neg<Output = T>,
    {
        match self {
            Direction::Right => Pt::new(one(), zero()),
            Direction::Down => Pt::new(zero(), -one::<T>()),
            Direction::Left => Pt::new(-one::<T>(), zero()),
            Direction::Up => Pt::new(zero(), one()),
        }
    }
}
