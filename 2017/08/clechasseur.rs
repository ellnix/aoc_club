use std::cmp::{max, Ordering};
use std::collections::HashMap;

use crate::input::day_8::INPUT;

pub fn part_1() -> i64 {
    final_registers().values().max().unwrap()
}

pub fn part_2() -> i64 {
    final_registers().max_ever
}

fn final_registers() -> Registers<'static> {
    let mut registers = Registers::default();
    INPUT
        .iter()
        .cloned()
        .map(Into::<Instruction>::into)
        .for_each(|instruction| instruction.apply(&mut registers));
    registers
}

#[derive(Debug, Default)]
struct Registers<'a> {
    registers: HashMap<&'a str, i64>,
    max_ever: i64,
}

impl<'a> Registers<'a> {
    fn get(&self, name: &'a str) -> i64 {
        self.registers.get(name).copied().unwrap_or_default()
    }

    fn update<F>(&mut self, name: &'a str, f: F)
    where
        F: FnOnce(i64) -> i64,
    {
        let register = self.registers.entry(name).or_default();
        *register = f(*register);
        self.max_ever = max(self.max_ever, *register);
    }

    fn values(&self) -> impl Iterator<Item = i64> + '_ {
        self.registers.values().copied()
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    register: &'a str,
    offset: i64,
    cmp_register: &'a str,
    cmp: Comparison,
    cmp_value: i64,
}

impl<'a> Instruction<'a> {
    fn apply(&self, registers: &mut Registers<'a>) {
        let cmp_register = registers.get(self.cmp_register);
        if self.cmp.apply(cmp_register, self.cmp_value) {
            registers.update(self.register, |r| r + self.offset);
        }
    }
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(value: &'a str) -> Self {
        let [register, op, offset, _, cmp_register, cmp, cmp_value] = value
            .split_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let offset = offset
            .parse::<i64>()
            .map(|o| if op == "dec" { -o } else { o })
            .unwrap();

        Self {
            register,
            offset,
            cmp_register,
            cmp: cmp.into(),
            cmp_value: cmp_value.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Comparison(Vec<Ordering>);

impl Comparison {
    const MATCHING_ORDERINGS: &'static [(char, Ordering)] =
        &[('<', Ordering::Less), ('>', Ordering::Greater), ('=', Ordering::Equal)];

    fn apply(&self, a: i64, b: i64) -> bool {
        self.0.contains(&a.cmp(&b))
    }
}

impl From<&str> for Comparison {
    fn from(value: &str) -> Self {
        Self(if value == "!=" {
            vec![Ordering::Less, Ordering::Greater]
        } else {
            Self::MATCHING_ORDERINGS
                .iter()
                .filter_map(|&(c, ord)| value.contains(c).then_some(ord))
                .collect()
        })
    }
}
