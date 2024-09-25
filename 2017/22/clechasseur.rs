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

type Node = Pt;

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
struct Cluster(Vec<NodeState>);

impl Cluster {
    pub fn new() -> Self {
        let mut nodes = Vec::new();
        nodes.resize(1_000 * 1_000, NodeState::Clean);
        Self(nodes)
    }

    pub fn get_state(&self, node: &Node) -> NodeState {
        self.0
            .get(Self::index(node))
            .copied()
            .unwrap_or(NodeState::Clean)
    }

    pub fn modify_state(&mut self, node: &Node, evolved: bool) -> NodeState {
        let next_state = self.get_state(node).next(evolved);
        self.set_state(node, next_state);
        next_state
    }

    fn index(node: &Node) -> usize {
        ((node.x + 500) * 1_000 + node.y + 500) as usize
    }

    fn set_state(&mut self, node: &Node, state: NodeState) {
        let state_ref = self
            .0
            .get_mut(Self::index(node))
            .expect("cluster size is not big enough for puzzle");
        *state_ref = state;
    }
}

impl FromStr for Cluster {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cluster = Self::new();

        s.lines().enumerate().for_each(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, node)| node == b'#')
                .for_each(|(x, _)| {
                    let node = Node::new(x as i64, y as i64);
                    cluster.set_state(&node, NodeState::Infected);
                });
        });

        Ok(cluster)
    }
}

impl Default for Cluster {
    fn default() -> Self {
        INPUT.parse().unwrap()
    }
}

type Carrier = Turtle;

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
