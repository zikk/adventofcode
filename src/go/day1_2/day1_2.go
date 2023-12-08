package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type DigitWithPosition struct {
	pos   int
	digit int
}

func main() {
	log.SetPrefix("Day 1 part 2")
	log.SetFlags(0)

	var envFlag string
	var inputFileName string

	numbers := [9]string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

	flag.StringVar(&envFlag, "e", "prod", "Input environment to use [test, prod]")
	flag.StringVar(&envFlag, "environment", "prod", "Input environment to use [test, prod]")
	flag.Parse()

	if envFlag == "prod" {
		inputFileName = "./inputs/day1.prod"
	} else {
		inputFileName = "./inputs/day1_2.test"
	}

	file, err := os.Open(inputFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	fileScanner := bufio.NewScanner(file)

	var total int

	for fileScanner.Scan() {
		line := fileScanner.Text()
		firstDigit := DigitWithPosition{-1, -1}
		lastDigit := DigitWithPosition{-1, -1}

		for charPos, char := range line {
			digit, charConvertionError := strconv.Atoi(string(char))

			if charConvertionError == nil {
				if firstDigit.digit == -1 {
					firstDigit = DigitWithPosition{charPos, digit}
				} else {
					lastDigit = DigitWithPosition{charPos, digit}
				}
			}
		}

		if lastDigit.digit == -1 {
			lastDigit = DigitWithPosition{firstDigit.pos, firstDigit.digit}
		}

		for index, number := range numbers {
			numberValue := index + 1
			firstOccurence := strings.Index(line, number)
			lastOccurence := strings.LastIndex(line, number)

			if firstOccurence > -1 {
				if firstDigit.pos == -1 || firstOccurence < firstDigit.pos {
					firstDigit = DigitWithPosition{firstOccurence, numberValue}
				}

				if lastDigit.pos == -1 || firstOccurence > lastDigit.pos {
					lastDigit = DigitWithPosition{firstOccurence, numberValue}
				}
			}

			if lastOccurence > -1 {
				if firstDigit.pos == -1 || lastOccurence < firstDigit.pos {
					firstDigit = DigitWithPosition{lastOccurence, numberValue}
				}

				if lastDigit.pos == -1 || lastOccurence > lastDigit.pos {
					lastDigit = DigitWithPosition{lastOccurence, numberValue}
				}
			}
		}

		if firstDigit.digit != -1 {
			lineTotal, error := strconv.Atoi((strconv.Itoa(firstDigit.digit) + strconv.Itoa(lastDigit.digit)))

			if error != nil {
				log.Fatal(error)
			}

			total = total + lineTotal
		}
	}

	fmt.Println(total)
}
