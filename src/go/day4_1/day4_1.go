package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"os"
	"regexp"
	"strings"
)

func main() {
	log.SetPrefix("Day 3 part 1")
	log.SetFlags(0)

	var envFlag string
	var inputFileName string

	flag.StringVar(&envFlag, "e", "prod", "Input environment to use [test, prod]")
	flag.StringVar(&envFlag, "environment", "prod", "Input environment to use [test, prod]")
	flag.Parse()

	if envFlag == "prod" {
		inputFileName = "./inputs/day4.in"
	} else {
		inputFileName = "./inputs/day4.test.in"
	}

	file, err := os.Open(inputFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	fileScanner := bufio.NewScanner(file)
	total := 0

	for fileScanner.Scan() {
		winnings := 0
		line := fileScanner.Text()
		numbers := strings.Split(strings.Split(line, ":")[1], "|")

		for _, n := range strings.Split(strings.ReplaceAll(strings.TrimSpace(numbers[1]), "  ", " "), " ") {
			var hasWinningNumber bool
			re := regexp.MustCompile("\\b" + string(n) + "\\b")
			f := re.FindStringIndex(strings.TrimSpace(numbers[0]))

			if len(f) > 0 {
				hasWinningNumber = true
			}

			if hasWinningNumber {
				if winnings == 0 {
					winnings = 1
				} else {
					winnings *= 2
				}
			}
		}

		total += winnings
	}

	fmt.Println(total)
}
