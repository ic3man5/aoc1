package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type Choice uint32

const (
	Rock Choice = iota + 1
	Paper
	Sissors
)

type Result uint32

const (
	Lost Result = 0
	Draw Result = 3
	Win  Result = 6
)

func main() {
	my_choice_map := make(map[string]Choice)
	my_choice_map["X"] = Rock
	my_choice_map["Y"] = Paper
	my_choice_map["Z"] = Sissors

	their_choice_map := make(map[string]Choice)
	their_choice_map["A"] = Rock
	their_choice_map["B"] = Paper
	their_choice_map["C"] = Sissors

	// Open the file
	f, err := os.Open("guide.txt")
	check(err)
	buffer := bufio.NewScanner(f)
	my_score := uint32(0)
	for buffer.Scan() {
		line := buffer.Text()
		round_choices := strings.Split(line, " ")
		if len(round_choices) != 2 {
			panic("Failed to parse round choice!")
		}
		their_choice := their_choice_map[round_choices[0]]
		my_choice := my_choice_map[round_choices[1]]
		round_result := uint32(0)
		if my_choice == Rock && their_choice == Rock {
			round_result += uint32(Draw)
		} else if my_choice == Rock && their_choice == Paper {
			round_result += uint32(Lost)
		} else if my_choice == Rock && their_choice == Sissors {
			round_result += uint32(Win)
		} else if my_choice == Paper && their_choice == Rock {
			round_result += uint32(Win)
		} else if my_choice == Paper && their_choice == Paper {
			round_result += uint32(Draw)
		} else if my_choice == Paper && their_choice == Sissors {
			round_result += uint32(Lost)
		} else if my_choice == Sissors && their_choice == Rock {
			round_result += uint32(Lost)
		} else if my_choice == Sissors && their_choice == Paper {
			round_result += uint32(Win)
		} else if my_choice == Sissors && their_choice == Sissors {
			round_result += uint32(Draw)
		} else {
			panic("Failed to parse round result from guide.txt\n")
		}

		my_score += round_result
		my_score += uint32(my_choice)
	}
	fmt.Printf("Total Score: %d!\n", my_score)

}
