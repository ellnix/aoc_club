use std::collections::HashMap;
use std::str::FromStr;

use strum::{EnumCount, FromRepr};

use crate::helpers::direction::Direction;
use crate::helpers::pt::Pt;
use crate::helpers::turtle::Turtle;
use crate::input::day_22::{CARRIER_START_POS, INPUT};

pub fn part_1() -> usize {
    infections_after(10_000, false)
}

pub fn part_2() -> usize {
    infections_after(10_000_000, true)
}

fn infections_after(bursts: usize, evolved: bool) -> usize {
    let mut state = State::new(evolved);
    for _ in 0..bursts {
        state.burst();
    }

    state.infections
}

type Node = Pt<isize>;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromRepr, EnumCount)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeState {
    pub fn next(&self, evolved: bool) -> Self {
        let jumps = if evolved { 1 } else { 2 };

        Self::from_repr(((*self as usize) + jumps) % Self::COUNT).unwrap()
    }
}

#[derive(Debug)]
struct Cluster(HashMap<Node, NodeState>);

impl Cluster {
    pub fn get_state(&self, node: &Node) -> NodeState {
        self.0.get(node).copied().unwrap_or(NodeState::Clean)
    }

    pub fn modify_state(&mut self, node: &Node, evolved: bool) -> NodeState {
        let next_state = self.get_state(node).next(evolved);
        self.0.insert(*node, next_state);
        next_state
    }
}

impl FromStr for Cluster {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .rev() // It's opposite day
                .enumerate()
                .flat_map(|(y, row)| {
                    row.bytes()
                        .enumerate()
                        .filter(|&(_, node)| node == b'#')
                        .map(move |(x, _)| (Node::new(x as isize, y as isize), NodeState::Infected))
                })
                .collect(),
        ))
    }
}

impl Default for Cluster {
    fn default() -> Self {
        INPUT.parse().unwrap()
    }
}

type Carrier = Turtle<isize>;

#[derive(Debug)]
struct State {
    cluster: Cluster,
    carrier: Carrier,
    evolved: bool,
    pub infections: usize,
}

impl State {
    pub fn new(evolved: bool) -> Self {
        Self {
            cluster: Cluster::default(),
            evolved,
            carrier: Carrier::new(CARRIER_START_POS, Direction::Up),
            infections: 0,
        }
    }

    pub fn burst(&mut self) {
        self.turn_carrier();
        if self
            .cluster
            .modify_state(&self.carrier.position, self.evolved)
            == NodeState::Infected
        {
            self.infections += 1;
        }
        self.carrier = self.carrier.advance();
    }

    fn turn_carrier(&mut self) {
        self.carrier = match self.cluster.get_state(&self.carrier.position) {
            NodeState::Clean => self.carrier.turn_left(),
            NodeState::Weakened => self.carrier,
            NodeState::Infected => self.carrier.turn_right(),
            NodeState::Flagged => self.carrier.turn_around(),
        }
    }
}
