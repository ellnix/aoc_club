package main

import (
	"strconv"
	"time"
)

type instruction struct {
	operator string
	operands []string
}

type cpu struct {
	instructions []instruction
	cpuid        int
	pc           int
	registers    map[string]int
	send         chan int
	recv         chan int
	sent         int
}

func newCPU(inst []instruction, cpuid int, snd chan int, rcv chan int) *cpu {
	return &cpu{
		registers:    map[string]int{"p": cpuid},
		send:         snd,
		recv:         rcv,
		instructions: inst,
		cpuid:        cpuid,
	}
}

func (c *cpu) getValue(operand string) int {
	if i, err := strconv.Atoi(operand); err == nil {
		return i
	}
	return c.registers[operand]
}

func (c *cpu) run() int {
	for {
		instruction := c.instructions[c.pc]
		switch instruction.operator {
		case "set":
			c.registers[instruction.operands[0]] = c.getValue(instruction.operands[1])
		case "add":
			c.registers[instruction.operands[0]] += c.getValue(instruction.operands[1])
		case "mul":
			c.registers[instruction.operands[0]] *= c.getValue(instruction.operands[1])
		case "mod":
			c.registers[instruction.operands[0]] %= c.getValue(instruction.operands[1])
		case "jgz":
			if c.getValue(instruction.operands[0]) > 0 {
				c.pc += c.getValue(instruction.operands[1])
				continue
			}
		case "snd":
			if c.send == nil {
				c.registers["sound"] = c.getValue(instruction.operands[0])
			} else {
				c.sent++
				c.send <- c.getValue(instruction.operands[0])
			}
		case "rcv":
			if c.send == nil {
				return c.registers["sound"]
			}
			select {
			case val := <-c.recv:
				c.registers[instruction.operands[0]] = val
			case <-time.After(100 * time.Millisecond):
				return c.sent
			}

		}
		c.pc++
	}

}

func part1(instructions []instruction) int {
	cpu0 := newCPU(instructions, 0, nil, nil)
	return cpu0.run()
}

func part2(instructions []instruction) int {
	zero2one := make(chan int, 100)
	one2zero := make(chan int, 100)
	cpu0 := newCPU(instructions, 0, one2zero, zero2one)
	cpu1 := newCPU(instructions, 1, zero2one, one2zero)
	cpu1.cpuid = 1
	go cpu0.run()
	return cpu1.run()
}
