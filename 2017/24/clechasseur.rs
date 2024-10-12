use std::convert::Infallible;
use std::str::FromStr;

use itertools::Itertools;

use crate::input::day_24::INPUT;

pub fn part_1() -> usize {
    bridges(vec![], initial_parts())
        .into_iter()
        .map(bridge_strength)
        .max()
        .unwrap()
}

pub fn part_2() -> usize {
    bridges(vec![], initial_parts())
        .into_iter()
        .max_by(|b1, b2| {
            b1.len()
                .cmp(&b2.len())
                .then_with(|| bridge_strength(b1).cmp(&bridge_strength(b2)))
        })
        .map(bridge_strength)
        .unwrap()
}

fn bridges(bridge: Vec<Part>, parts: Vec<Part>) -> Vec<Vec<Part>> {
    let connect_to = bridge.last().map(|part| part.1).unwrap_or_default();
    let candidates = parts
        .iter()
        .enumerate()
        .filter_map(|(i, part)| {
            part.connecting(connect_to)
                .map(|connecting_part| (i, connecting_part))
        })
        .collect_vec();
    if candidates.is_empty() {
        return vec![bridge];
    }

    candidates
        .into_iter()
        .flat_map(|(i, part)| {
            let mut new_bridge = bridge.clone();
            new_bridge.push(part);
            let mut new_parts = parts.clone();
            new_parts.remove(i);

            bridges(new_bridge, new_parts)
        })
        .collect_vec()
}

fn bridge_strength<P>(parts: P) -> usize
where
    P: AsRef<[Part]>,
{
    parts.as_ref().iter().map(Part::strength).sum()
}

#[derive(Debug, Copy, Clone)]
struct Part(pub usize, pub usize);

impl Part {
    pub fn opposite(&self) -> Self {
        Self(self.1, self.0)
    }

    pub fn connecting(&self, to: usize) -> Option<Self> {
        if to == self.0 {
            Some(*self)
        } else if to == self.1 {
            Some(self.opposite())
        } else {
            None
        }
    }

    pub fn strength(&self) -> usize {
        self.0 + self.1
    }
}

impl FromStr for Part {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split('/')
            .map(|strength| strength.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self(left, right))
    }
}

fn initial_parts() -> Vec<Part> {
    INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec()
}
