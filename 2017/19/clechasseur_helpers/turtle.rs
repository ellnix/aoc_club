use std::fmt::{Display, Formatter};
use std::ops::{Add, Neg};

use num::{zero, One, Zero};

use crate::helpers::direction::Direction;
use crate::helpers::pt::Pt;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Turtle<T> {
    pub position: Pt<T>,
    pub direction: Direction,
}

impl<T> Turtle<T> {
    pub fn new(position: Pt<T>, direction: Direction) -> Self {
        Self { position, direction }
    }

    pub fn from_zero(direction: Direction) -> Self
    where
        Pt<T>: Zero,
    {
        Self::new(zero(), direction)
    }

    pub fn turn_left(&self) -> Self
    where
        Pt<T>: Copy,
    {
        Self { direction: self.direction.turn_left(), ..*self }
    }

    pub fn turn_right(&self) -> Self
    where
        Pt<T>: Copy,
    {
        Self { direction: self.direction.turn_right(), ..*self }
    }

    pub fn advance(&self) -> Self
    where
        T: Zero + One + Neg<Output = T> + Add<Output = T>,
        Pt<T>: Copy,
    {
        Self { position: self.position + self.direction.displacement(), ..*self }
    }
}

impl<T> Display for Turtle<T>
where
    Pt<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ position: {}, direction: {} }}", self.position, self.direction)
    }
}
