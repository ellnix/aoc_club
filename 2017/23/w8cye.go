package main

// Assembly code changes
// diff orig.txt input.txt
// 16c16
// < set f 0
// ---
// > jnz 1 10
// 23c23
// < sub g b
// ---
// > sub g 357

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

type Opcode int

const (
	Set Opcode = iota
	Mul
	Jnz
	Sub
)

type OperandType int

const (
	Value OperandType = iota
	Register
)

type Operand struct {
	Type     OperandType
	Value    int
	Register int
}
type Instruction struct {
	Opcode Opcode
	Op1    Operand
	Op2    Operand
	Text   string
}

type cpu struct {
	instructions []Instruction
	pc           int
	registers    []int
	mulCount     int
}

func newCPU(inst []Instruction) *cpu {
	return &cpu{
		registers:    make([]int, 8), // registers a-h
		instructions: inst,
	}
}

func (o *Operand) getValue(register []int) int {
	if o.Type == Value {
		return o.Value
	}
	return register[o.Register]
}

func (c *cpu) run() {
	for c.pc >= 0 && c.pc < len(c.instructions) {
		instruction := &c.instructions[c.pc]
		switch instruction.Opcode {
		case Set:
			c.registers[instruction.Op1.Register] = instruction.Op2.getValue(c.registers)
		case Sub:
			// if instruction.Op1.Register == 7 {
			// 	fmt.Println(c.registers)
			// }
			c.registers[instruction.Op1.Register] -= instruction.Op2.getValue(c.registers)
		case Mul:
			c.registers[instruction.Op1.Register] *= instruction.Op2.getValue(c.registers)
			c.mulCount++
		case Jnz:
			if instruction.Op1.getValue(c.registers) != 0 {
				c.pc += instruction.Op2.getValue(c.registers)
				continue
			}
		}
		c.pc++
	}
}

func part1(instructions []Instruction) int {
	cpu0 := newCPU(instructions)
	cpu0.run()
	return cpu0.mulCount
}

func part2(instructions []Instruction) int {
	cpu0 := newCPU(instructions)
	cpu0.registers[0] = 1 // set register a to 1
	cpu0.run()
	return cpu0.registers[7] // return value in register h
}

func parseInstructionsFromFile(filename string) ([]Instruction, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, fmt.Errorf("error opening file: %v", err)
	}
	defer file.Close()

	var instructions []Instruction
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		instruction, err := parseInstruction(line)
		if err != nil {
			return nil, fmt.Errorf("error parsing line: %v", err)
		}
		instructions = append(instructions, instruction)
	}

	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("error reading file: %v", err)
	}

	return instructions, nil
}

func mapOpcode(opcode string) Opcode {
	switch opcode {
	case "set":
		return Set
	case "mul":
		return Mul
	case "jnz":
		return Jnz
	case "sub":
		return Sub
	default:
		return -1
	}
}

func mapOperand(operand string) Operand {
	if i, err := strconv.Atoi(operand); err == nil {
		return Operand{Type: Value, Value: i}
	}
	return Operand{Type: Register, Register: int(operand[0]) - 97}
}

func parseInstruction(line string) (Instruction, error) {
	fields := strings.Fields(line)
	if len(fields) == 0 {
		return Instruction{}, fmt.Errorf("empty line")
	}
	instruction := Instruction{
		Opcode: mapOpcode(fields[0]),
		Op1:    mapOperand(fields[1]),
		Op2:    mapOperand(fields[2]),
		Text:   line,
	}

	return instruction, nil
}

func main() {
	filename := "input.txt"
	instructions, err := parseInstructionsFromFile(filename)
	if err != nil {
		fmt.Printf("Error parsing instructions: %v\n", err)
		os.Exit(1)
	}

	t := time.Now()
	p1 := part1(instructions)
	t1 := time.Since(t)

	fmt.Printf("Part 1: %5d - Execution Time: %s\n", p1, t1.String())

	t = time.Now()
	p2 := part2(instructions)
	t2 := time.Since(t)

	fmt.Printf("Part 2: %5d - Execution Time: %s\n", p2, t2.String())
}
