package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	log.SetPrefix("Day 1 part 1")
	log.SetFlags(0)

	var envFlag string
	var inputFileName string

	flag.StringVar(&envFlag, "e", "prod", "Input environment to use [test, prod]")
	flag.StringVar(&envFlag, "environment", "prod", "Input environment to use [test, prod]")
	flag.Parse()

	if envFlag == "prod" {
		inputFileName = "./inputs/day1.in"
	} else {
		inputFileName = "./inputs/day1_1.test.in"
	}

	file, err := os.Open(inputFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var total int

	for scanner.Scan() {
		line := scanner.Text()
		var firstDigit int
		var lastDigit int

		for _, char := range line {
			digit, charConvertionError := strconv.Atoi(string(char))

			if charConvertionError == nil {
				if firstDigit == 0 {
					firstDigit = digit
				} else {
					lastDigit = digit
				}
			}
		}

		if firstDigit != 0 {
			var lineTotal int
			var error error

			if lastDigit != 0 {
				lineTotal, error = strconv.Atoi((strconv.Itoa(firstDigit) + strconv.Itoa(lastDigit)))

				if error != nil {
					log.Fatal(error)
				}
			} else {
				lineTotal, error = strconv.Atoi((strconv.Itoa(firstDigit) + strconv.Itoa(firstDigit)))

				if error != nil {
					log.Fatal(error)
				}
			}

			total = total + lineTotal
		}
	}

	fmt.Println(total)
}
