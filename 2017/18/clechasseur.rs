use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::str::FromStr;

use anyhow::{anyhow, Context};

use crate::input::day_18::INPUT;

pub fn part_1() -> i64 {
    let program: Program = INPUT.parse().unwrap();
    let mut interpreter = DuetInterpreter::for_part_1(program);

    loop {
        if let Some(n) = interpreter.execute_next().unwrap().received() {
            return n;
        }
    }
}

pub fn part_2() -> usize {
    let program: Program = INPUT.parse().unwrap();

    let program_0_queue = Rc::new(RefCell::new(Queue::default()));
    let program_1_queue = Rc::new(RefCell::new(Queue::default()));

    let mut interpreters = [
        DuetInterpreter::for_part_2(
            program.clone(),
            0,
            Rc::clone(&program_1_queue),
            Rc::clone(&program_0_queue),
        ),
        DuetInterpreter::for_part_2(
            program.clone(),
            1,
            Rc::clone(&program_0_queue),
            Rc::clone(&program_1_queue),
        ),
    ];

    let mut interpreter = 0usize;
    let mut wait_count = 0;
    loop {
        let next_interpreter = &mut interpreters[interpreter];
        loop {
            match next_interpreter.execute_next().unwrap() {
                InstructionResult::Waiting => break,
                _ => wait_count = 0,
            }
        }

        wait_count += 1;
        if wait_count == 2 {
            return interpreters[1].send_count();
        }
        interpreter = (interpreter + 1) % interpreters.len();
    }
}

#[derive(Debug, Default)]
struct Queue(VecDeque<i64>);

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
struct Registers(HashMap<char, i64>);

impl Registers {
    pub fn get(&self, register: char) -> i64 {
        self.0.get(&register).copied().unwrap_or_default()
    }

    pub fn set(&mut self, register: char, value: i64) {
        self.0.insert(register, value);
    }
}

#[derive(Debug, Copy, Clone)]
enum Value {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum InstructionResult {
    Unit,
    JmpOffset(i64),
    Received(i64),
    Waiting,
}

impl InstructionResult {
    pub fn jmp_offset(&self) -> i64 {
        match self {
            Self::JmpOffset(offset) => *offset,
            Self::Waiting => 0,
            _ => 1,
        }
    }

    pub fn received(&self) -> Option<i64> {
        match self {
            Self::Received(n) => Some(*n),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
}

impl Instruction {
    pub fn execute(
        &self,
        registers: &mut Registers,
        send_count: &mut usize,
        send_queue: Rc<RefCell<Queue>>,
        rcv_queue: Rc<RefCell<Queue>>,
        part_1: bool,
    ) -> Result<InstructionResult, anyhow::Error> {
        match self {
            Self::Snd(value) => {
                *send_count += 1;
                send_queue.borrow_mut().push(value.get(registers));
            },
            Self::Set(register, value) => registers.set(*register, value.get(registers)),
            Self::Add(register, value) => {
                registers.set(*register, registers.get(*register) + value.get(registers))
            },
            Self::Mul(register, value) => {
                registers.set(*register, registers.get(*register) * value.get(registers))
            },
            Self::Mod(register, value) => {
                registers.set(*register, registers.get(*register).rem_euclid(value.get(registers)))
            },
            Self::Rcv(register) => {
                if part_1 {
                    if registers.get(*register) != 0 {
                        return Ok(InstructionResult::Received(
                            rcv_queue
                                .borrow_mut()
                                .pop_last()
                                .with_context(|| "no sound played")?,
                        ));
                    }
                } else {
                    return Ok(match rcv_queue.borrow_mut().pop() {
                        Some(n) => {
                            registers.set(*register, n);
                            InstructionResult::Received(n)
                        },
                        None => InstructionResult::Waiting,
                    });
                }
            },
            Self::Jgz(value, jmp_offset) => {
                if value.get(registers) > 0 {
                    return Ok(InstructionResult::JmpOffset(jmp_offset.get(registers)));
                }
            },
        }

        Ok(InstructionResult::Unit)
    }

    fn read_register<'a, I>(parts: &mut I) -> Result<char, anyhow::Error>
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

    fn read_value<'a, I>(parts: &mut I) -> Result<Value, anyhow::Error>
    where
        I: Iterator<Item = &'a str>,
    {
        parts.next().with_context(|| "missing value")?.parse()
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let opcode = parts.next().with_context(|| "empty instruction")?;

        match opcode {
            "snd" => Ok(Self::Snd(Self::read_value(&mut parts)?)),
            "set" => Ok(Self::Set(Self::read_register(&mut parts)?, Self::read_value(&mut parts)?)),
            "add" => Ok(Self::Add(Self::read_register(&mut parts)?, Self::read_value(&mut parts)?)),
            "mul" => Ok(Self::Mul(Self::read_register(&mut parts)?, Self::read_value(&mut parts)?)),
            "mod" => Ok(Self::Mod(Self::read_register(&mut parts)?, Self::read_value(&mut parts)?)),
            "rcv" => Ok(Self::Rcv(Self::read_register(&mut parts)?)),
            "jgz" => Ok(Self::Jgz(Self::read_value(&mut parts)?, Self::read_value(&mut parts)?)),
            opcode => Err(anyhow!("invalid opcode: {opcode}")),
        }
    }
}

#[derive(Debug, Clone)]
struct Program(Vec<Instruction>);

impl Program {
    pub fn execute(
        &self,
        ip: i64,
        registers: &mut Registers,
        send_count: &mut usize,
        send_queue: Rc<RefCell<Queue>>,
        rcv_queue: Rc<RefCell<Queue>>,
        part_1: bool,
    ) -> Result<InstructionResult, anyhow::Error> {
        self.0
            .get(ip as usize)
            .with_context(|| format!("invalid instruction pointer: {ip}"))?
            .execute(registers, send_count, send_queue, rcv_queue, part_1)
    }
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?))
    }
}

#[derive(Debug)]
struct DuetInterpreter {
    program: Program,
    registers: Registers,
    ip: i64,
    send_count: usize,
    send_queue: Rc<RefCell<Queue>>,
    rcv_queue: Rc<RefCell<Queue>>,
    part_1: bool,
}

impl DuetInterpreter {
    pub fn for_part_1(program: Program) -> Self {
        let send_queue = Rc::new(RefCell::new(Queue::default()));
        let rcv_queue = Rc::clone(&send_queue);

        Self {
            program,
            registers: Registers::default(),
            ip: 0,
            send_count: 0,
            send_queue,
            rcv_queue,
            part_1: true,
        }
    }

    pub fn for_part_2(
        program: Program,
        id: i64,
        send_queue: Rc<RefCell<Queue>>,
        rcv_queue: Rc<RefCell<Queue>>,
    ) -> Self {
        let mut registers = Registers::default();
        registers.set('p', id);

        Self { program, registers, ip: 0, send_count: 0, send_queue, rcv_queue, part_1: false }
    }

    pub fn execute_next(&mut self) -> Result<InstructionResult, anyhow::Error> {
        let result = self.program.execute(
            self.ip,
            &mut self.registers,
            &mut self.send_count,
            Rc::clone(&self.send_queue),
            Rc::clone(&self.rcv_queue),
            self.part_1,
        )?;
        self.ip += result.jmp_offset();
        Ok(result)
    }

    pub fn send_count(&self) -> usize {
        self.send_count
    }
}
