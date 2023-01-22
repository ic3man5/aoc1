package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	// Open the file
	f, err := os.Open("calories.txt")
	check(err)
	buffer := bufio.NewScanner(f)
	total_calories := 0
	all_calories := make([]int, 0)
	// Read all the lines in the file
	for buffer.Scan() {
		line := buffer.Text()
		// Append and reset the result
		if line == "" {
			all_calories = append(all_calories, total_calories)
			total_calories = 0
			continue
		}
		calorie, err := strconv.Atoi(line)
		check(err)
		total_calories += calorie
	}
	// Find the max value
	max_calorie := 0
	for _, cal := range all_calories {
		if cal > max_calorie {
			max_calorie = cal
		}
	}
	fmt.Printf("Elf with the highest calorie count is: %#v!\n", max_calorie)
}
