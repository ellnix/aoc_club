use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use itertools::Itertools;

use crate::input::day_7::INPUT; // &[ProgramSpec]

pub fn part_1() -> &'static str {
    Tower::build().bottom.borrow().name
}

pub fn part_2() -> usize {
    let (_imbalanced, balanced) = Tower::build().imbalance();
    balanced
}

#[derive(Debug)]
pub struct ProgramSpec<'a> {
    pub name: &'a str,
    pub weight: usize,
    pub sub_programs: &'a [&'a str],
}

#[derive(Debug, Default)]
struct Program<'a> {
    name: &'a str,
    weight: usize,
    sub_programs: Vec<Rc<RefCell<Program<'a>>>>,
}

impl<'a> Program<'a> {
    fn new(name: &'a str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { name, ..Self::default() }))
    }

    fn total_weight(&self, cache: &mut HashMap<&'a str, usize>) -> usize {
        match cache.get(self.name).copied() {
            Some(weight) => weight,
            None => {
                let weight = self.weight + self.sub_weight(cache);
                cache.insert(self.name, weight);
                weight
            },
        }
    }

    fn sub_weight(&self, cache: &mut HashMap<&'a str, usize>) -> usize {
        self.sub_programs
            .iter()
            .map(|sub_program| sub_program.borrow().total_weight(cache))
            .sum()
    }

    fn imbalance(&self, weights: &HashMap<&'a str, usize>) -> Option<(usize, usize)> {
        let sub_data: Vec<_> = self
            .sub_programs
            .iter()
            .map(|sub_program| (sub_program, sub_program.borrow().imbalance(weights)))
            .collect();

        match sub_data
            .iter()
            .filter(|(_, imbalance)| imbalance.is_some())
            .at_most_one()
        {
            Ok(Some(&(_, imbalance))) => imbalance,
            Ok(None) => {
                let programs: Vec<_> = sub_data
                    .into_iter()
                    .map(|(sub_program, _)| sub_program)
                    .sorted_by_key(|sub_program| weights[sub_program.borrow().name])
                    .dedup_by_with_count(|sp1, sp2| {
                        weights[sp1.borrow().name] == weights[sp2.borrow().name]
                    })
                    .sorted_by_key(|&(count, _)| count)
                    .map(|(_, sub_program)| sub_program)
                    .collect();

                (programs.len() == 2).then(|| {
                    let imbalance = weights[programs[0].borrow().name] as isize
                        - weights[programs[1].borrow().name] as isize;
                    let imbalanced_weight = programs[0].borrow().weight;
                    let balanced_weight = imbalanced_weight.checked_add_signed(-imbalance).unwrap();
                    (imbalanced_weight, balanced_weight)
                })
            },
            Err(_) => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Tower<'a> {
    bottom: Rc<RefCell<Program<'a>>>,
}

impl<'a> Tower<'a> {
    fn build() -> Self {
        let mut programs = HashMap::new();
        let mut parents = HashSet::new();

        for spec in INPUT {
            let program = Rc::clone(programs.entry(spec.name).or_insert_with(|| {
                parents.insert(spec.name);
                Program::new(spec.name)
            }));
            program.borrow_mut().weight = spec.weight;
            for &sub_prog_name in spec.sub_programs {
                let sub_program = Rc::clone(
                    programs
                        .entry(sub_prog_name)
                        .or_insert_with(|| Program::new(sub_prog_name)),
                );
                program.borrow_mut().sub_programs.push(sub_program);
                parents.remove(sub_prog_name);
            }
        }

        Tower {
            bottom: programs
                .remove(parents.into_iter().exactly_one().unwrap())
                .unwrap(),
        }
    }

    fn total_weights(&self) -> HashMap<&'a str, usize> {
        let mut weights = HashMap::new();
        let bottom_weight = self.bottom.borrow().total_weight(&mut weights);
        weights.insert(self.bottom.borrow().name, bottom_weight);
        weights
    }

    fn imbalance(&self) -> (usize, usize) {
        let weights = self.total_weights();
        self.bottom.borrow().imbalance(&weights).unwrap()
    }
}
