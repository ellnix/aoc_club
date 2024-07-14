use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;

use crate::input::day_12::INPUT;

pub fn part_1() -> usize {
    Village::from(INPUT).group(0).len()
}

pub fn part_2() -> usize {
    Village::from(INPUT).group_count()
}

#[derive(Debug)]
struct Program {
    id: usize,
    neighbours: Vec<usize>,
}

impl<'a> From<&'a str> for Program {
    fn from(value: &'a str) -> Self {
        let (id, neighbours) = value.split(" <-> ").collect_tuple().unwrap();
        Self {
            id: id.parse().unwrap(),
            neighbours: neighbours
                .split(", ")
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()
                .unwrap(),
        }
    }
}

#[derive(Debug)]
struct Village(HashMap<usize, Program>);

impl Village {
    pub fn group(&self, id: usize) -> HashSet<usize> {
        let mut group = HashSet::new();
        self.fill_group(id, &mut group);
        group
    }

    pub fn group_count(&self) -> usize {
        let mut seen = HashSet::new();
        let mut count = 0;

        self.0.keys().for_each(|&id| {
            if !seen.contains(&id) {
                seen.extend(self.group(id));
                count += 1;
            }
        });

        count
    }

    fn fill_group(&self, id: usize, group: &mut HashSet<usize>) {
        if !group.contains(&id) {
            let program = &self.0[&id];
            group.insert(program.id);
            program
                .neighbours
                .iter()
                .for_each(|&neighbour_id| self.fill_group(neighbour_id, group));
        }
    }
}

impl<'a, 'b> From<&'a [&'b str]> for Village {
    fn from(value: &'a [&'b str]) -> Self {
        Self(
            value
                .iter()
                .cloned()
                .map(Into::<Program>::into)
                .map(|p| (p.id, p))
                .collect(),
        )
    }
}
