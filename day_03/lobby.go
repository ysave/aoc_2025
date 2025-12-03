package main

import (
	"fmt"
	"bufio"
	"os"
)

func main() {
	file, err := os.Open("joltages.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	partOne(lines)
	partTwo(lines)
}


func partOne(lines []string) {
	sum := 0
	for _, line := range lines {
		var firstBatterie, secondBatterie int
		for i, char := range line {
			num := int(char - '0')

			if firstBatterie < num && i != len(line)-1 {
				firstBatterie = num
				secondBatterie = 0
			} else if secondBatterie < num {
				secondBatterie = num
			}
		}
		sum += firstBatterie * 10 + secondBatterie
	}
	fmt.Println("Solution part one:", sum)
}


func partTwo(lines []string) {
	
}