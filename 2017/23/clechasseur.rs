use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use primes::is_prime;
use strum::EnumDiscriminants;

use crate::helpers::duet::{read_instructions, read_register, read_value, Registers, Value};
use crate::input::day_23::INPUT;

pub fn part_1() -> usize {
    let program = Program::default();
    let mut coprocessor = ExperimentalCoprocessor::new(program);

    coprocessor.execute().unwrap();
    coprocessor.op_count(InstructionDiscriminants::Mul)
}

pub fn part_2() -> i64 {
    let program = Program::default().optimize();
    let mut coprocessor = ExperimentalCoprocessor::new(program);

    coprocessor.execute().unwrap();
    coprocessor.register('h')
}

#[derive(Debug, Copy, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
enum Instruction {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),

    // Non-standard instructions:
    Isp(char, Value), // IS_PRIME
}

impl Instruction {
    pub fn execute(
        &self,
        registers: &mut Registers,
        op_counts: &mut OpCounts,
    ) -> Result<InstructionResult, anyhow::Error> {
        op_counts.inc(self);

        match self {
            Self::Set(register, value) => registers.set(*register, value.get(registers)),
            Self::Sub(register, value) => {
                registers.set(*register, registers.get(*register) - value.get(registers))
            },
            Self::Mul(register, value) => {
                registers.set(*register, registers.get(*register) * value.get(registers))
            },
            Self::Jnz(value, jmp_offset) => {
                if value.get(registers) != 0 {
                    return Ok(InstructionResult::JmpOffset(jmp_offset.get(registers)));
                }
            },

            // Non-standard instructions:
            Self::Isp(register, value) => {
                registers.set(*register, if is_prime(value.get(registers) as u64) { 1 } else { 0 });
            },
        }

        Ok(InstructionResult::Unit)
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let opcode = parts.next().with_context(|| "empty instruction")?;

        match opcode {
            "set" => Ok(Self::Set(read_register(&mut parts)?, read_value(&mut parts)?)),
            "sub" => Ok(Self::Sub(read_register(&mut parts)?, read_value(&mut parts)?)),
            "mul" => Ok(Self::Mul(read_register(&mut parts)?, read_value(&mut parts)?)),
            "jnz" => Ok(Self::Jnz(read_value(&mut parts)?, read_value(&mut parts)?)),

            // Non-standard instructions:
            "isp" => Ok(Self::Isp(read_register(&mut parts)?, read_value(&mut parts)?)),

            opcode => Err(anyhow!("invalid opcode: {opcode}")),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InstructionResult {
    Unit,
    JmpOffset(i64),
    Exited,
}

impl InstructionResult {
    pub fn jmp_offset(&self) -> i64 {
        match self {
            Self::JmpOffset(offset) => *offset,
            _ => 1,
        }
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn execute(
        &self,
        ip: i64,
        registers: &mut Registers,
        op_counts: &mut OpCounts,
    ) -> Result<InstructionResult, anyhow::Error> {
        usize::try_from(ip)
            .ok()
            .and_then(|ip| self.instructions.get(ip))
            .map(|instruction| instruction.execute(registers, op_counts))
            .unwrap_or(Ok(InstructionResult::Exited))
    }
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { instructions: read_instructions(s)? })
    }
}

impl Default for Program {
    fn default() -> Self {
        INPUT.parse().unwrap()
    }
}

#[derive(Debug, Default)]
struct OpCounts(HashMap<InstructionDiscriminants, usize>);

impl OpCounts {
    pub fn inc<I>(&mut self, instruction: I)
    where
        I: Into<InstructionDiscriminants>,
    {
        *self.0.entry(instruction.into()).or_default() += 1;
    }

    pub fn count<I>(&self, instruction: I) -> usize
    where
        I: Into<InstructionDiscriminants>,
    {
        self.0.get(&instruction.into()).copied().unwrap_or_default()
    }
}

#[derive(Debug)]
struct ExperimentalCoprocessor {
    program: Program,
    registers: Registers,
    ip: i64,
    op_counts: OpCounts,
}

impl ExperimentalCoprocessor {
    pub fn new(program: Program) -> Self {
        Self { program, registers: Registers::default(), ip: 0, op_counts: OpCounts::default() }
    }

    pub fn execute_next(&mut self) -> Result<InstructionResult, anyhow::Error> {
        let result = self
            .program
            .execute(self.ip, &mut self.registers, &mut self.op_counts)?;
        self.ip += result.jmp_offset();
        Ok(result)
    }

    pub fn execute(&mut self) -> Result<(), anyhow::Error> {
        while self.execute_next()? != InstructionResult::Exited {}

        Ok(())
    }

    pub fn op_count<I>(&self, instruction: I) -> usize
    where
        I: Into<InstructionDiscriminants>,
    {
        self.op_counts.count(instruction)
    }

    pub fn register(&self, register: char) -> i64 {
        self.registers.get(register)
    }
}

/// The initial program counts the number of non-prime numbers
/// between two numbers (inclusive), stepping by 17.
///
/// This version is "optimized", but it uses a non-standard
/// instruction (`isp`) to determine if a number is prime.
///
/// To use, keep the program's first instruction and replace
/// the rest with this.
const OPTIMIZED: &str = "mul b 100\n\
                         sub b -100000\n\
                         set c b\n\
                         sub c -17000\n\
                         isp f b\n\
                         jnz f 2\n\
                         sub h -1\n\
                         set g b\n\
                         sub g c\n\
                         jnz g 2\n\
                         jnz 1 42\n\
                         sub b -17\n\
                         jnz g -8";

impl Program {
    pub fn optimize(mut self) -> Self {
        let mut optimized: Program = OPTIMIZED.parse().unwrap();
        optimized
            .instructions
            .insert(0, self.instructions.remove(0));
        optimized
    }
}
