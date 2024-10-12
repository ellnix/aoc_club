package main

// Inspired by IsaacG

import (
	"strconv"
	"strings"
)

type magneticComponent struct {
	leftPort  int
	rightPort int
}

func (m *magneticComponent) getOtherPort(connectedPort int) (int, bool) {
	switch connectedPort {
	case m.leftPort:
		return m.rightPort, true
	case m.rightPort:
		return m.leftPort, true
	default:
		return 0, false
	}
}

func (m *magneticComponent) strength() int {
	return m.leftPort + m.rightPort
}

type Day24 struct {
	components []magneticComponent
}

func (d *Day24) findMaxStrength(nextPortType int, components []magneticComponent) (maxBridgeStrength, maxBridgeLength, maxStrengthOfLongestBridge int) {
	for i := range components {
		other, ok := components[i].getOtherPort(nextPortType)
		if !ok {
			continue
		}

		components[0], components[i] = components[i], components[0]
		bridgeStrength, bridgeLength, strengthOfLongestBridge := d.findMaxStrength(other, components[1:])
		components[0], components[i] = components[i], components[0]

		strengthOfComponent := components[i].strength()
		maxBridgeStrength = max(maxBridgeStrength, strengthOfComponent+bridgeStrength)

		switch {
		case bridgeLength > maxBridgeLength:
			maxBridgeLength = bridgeLength
			maxStrengthOfLongestBridge = strengthOfLongestBridge + strengthOfComponent
		case bridgeLength == maxBridgeLength:
			maxStrengthOfLongestBridge = max(maxStrengthOfLongestBridge, strengthOfLongestBridge+strengthOfComponent)
		}

	}
	return maxBridgeStrength, maxBridgeLength + 1, maxStrengthOfLongestBridge
}

func parseInput(lines []string) []magneticComponent {
	components := make([]magneticComponent, 0, len(lines))
	for _, line := range lines {
		portTypes := strings.Split(line, "/")
		leftPort, _ := strconv.Atoi(portTypes[0])
		rightPort, _ := strconv.Atoi(portTypes[1])
		components = append(components, magneticComponent{leftPort, rightPort})
	}
	return components
}

func NewDay24(lines []string) *Day24 {
	return &Day24{parseInput(lines)}
}

func (d *Day24) part1() int {
	part1, _, _ := d.findMaxStrength(0, d.components)
	return part1
}

func (d *Day24) part2() int {
	_, _, part2 := d.findMaxStrength(0, d.components)
	return part2
}
