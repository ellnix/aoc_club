use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use anyhow::Context;

#[derive(Debug, Default)]
pub struct Queue(VecDeque<i64>);

impl Queue {
    pub fn push(&mut self, value: i64) {
        self.0.push_back(value)
    }

    pub fn pop(&mut self) -> Option<i64> {
        self.0.pop_front()
    }

    pub fn pop_last(&mut self) -> Option<i64> {
        self.0.pop_back()
    }
}

#[derive(Debug, Default)]
pub struct Registers(HashMap<char, i64>);

impl Registers {
    pub fn get(&self, register: char) -> i64 {
        self.0.get(&register).copied().unwrap_or_default()
    }

    pub fn set(&mut self, register: char, value: i64) {
        self.0.insert(register, value);
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Number(i64),
    Register(char),
}

impl Value {
    pub fn get(&self, registers: &Registers) -> i64 {
        match self {
            Self::Number(n) => *n,
            Self::Register(register) => registers.get(*register),
        }
    }
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i64>() {
            Ok(n) => Self::Number(n),
            Err(_) => Self::Register(s.chars().next().with_context(|| "empty value")?),
        })
    }
}

pub fn read_register<'a, I>(parts: &mut I) -> Result<char, anyhow::Error>
where
    I: Iterator<Item = &'a str>,
{
    parts
        .next()
        .with_context(|| "missing register name")?
        .chars()
        .next()
        .with_context(|| "empty register name")
}

pub fn read_value<'a, I>(parts: &mut I) -> Result<Value, anyhow::Error>
where
    I: Iterator<Item = &'a str>,
{
    parts.next().with_context(|| "missing value")?.parse()
}

pub fn read_instructions<T>(s: &str) -> Result<Vec<T>, anyhow::Error>
where
    T: FromStr<Err = anyhow::Error>,
{
    s.lines().map(str::parse).collect()
}
