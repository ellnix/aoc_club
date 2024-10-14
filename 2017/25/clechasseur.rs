use std::collections::HashMap;
use std::rc::Rc;

use bit_vec::BitVec;
use serde::Deserialize;

use crate::input::day_25::{CHECKSUM_AFTER, STARTING_STATE, STATES};

pub fn part_1() -> usize {
    let mut machine = TuringMachine::default();
    machine.run_until_checksum();
    machine.checksum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TuringMove {
    Left,
    Right,
}

impl TuringMove {
    pub fn displacement(&self) -> isize {
        match self {
            TuringMove::Left => -1,
            TuringMove::Right => 1,
        }
    }

    pub fn apply(&self, machine: &mut TuringMachine) {
        machine.cursor += self.displacement();
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TuringOp {
    pub value: usize,
    pub movement: TuringMove,
    pub next_state: String,
}

impl TuringOp {
    pub fn apply(&self, machine: &mut TuringMachine) {
        machine.tape.set(machine.cursor, self.value);
        self.movement.apply(machine);
        machine.current_state = self.next_state.clone();
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct TuringState([TuringOp; 2]);

impl TuringState {
    pub fn apply(&self, machine: &mut TuringMachine) {
        let current_value = machine.tape.get(machine.cursor);
        self.0[current_value].apply(machine);
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct TuringStates(HashMap<String, Rc<TuringState>>);

impl TuringStates {
    pub fn get(&self, name: &str) -> Rc<TuringState> {
        Rc::clone(self.0.get(name).unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct TuringTape(BitVec);

impl TuringTape {
    const CAPACITY: usize = 10_000;
    const OFFSET: isize = 4_000;

    pub fn get(&self, pos: isize) -> usize {
        self.0
            .get((pos + Self::OFFSET) as usize)
            .map(|b| if b { 1 } else { 0 })
            .unwrap_or_default()
    }

    pub fn set(&mut self, pos: isize, value: usize) {
        self.0.set((pos + Self::OFFSET) as usize, value != 0)
    }

    pub fn checksum(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl Default for TuringTape {
    fn default() -> Self {
        Self(BitVec::from_elem(Self::CAPACITY, false))
    }
}

#[derive(Debug, Clone)]
pub struct TuringMachine {
    states: TuringStates,
    checksum_after: usize,
    current_state: String,
    tape: TuringTape,
    cursor: isize,
    steps: usize,
}

impl TuringMachine {
    pub fn step(&mut self) {
        let state = self.states.get(&self.current_state);
        state.apply(self);
        self.steps += 1;
    }

    pub fn run_until_checksum(&mut self) {
        while self.steps < self.checksum_after {
            self.step();
        }
    }

    pub fn checksum(&self) -> usize {
        self.tape.checksum()
    }
}

impl Default for TuringMachine {
    fn default() -> Self {
        Self {
            states: serde_json::from_str(STATES).unwrap(),
            checksum_after: CHECKSUM_AFTER,
            current_state: STARTING_STATE.into(),
            tape: TuringTape::default(),
            cursor: 0,
            steps: 0,
        }
    }
}
