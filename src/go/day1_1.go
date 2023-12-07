package main

import (
	"flag"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	log.SetPrefix("Day 1 part 1")
	log.SetFlags(0)

	var envFlag string
	var inputFileName string

	flag.StringVar(&envFlag, "e", "prod", "Input environment to use [test, prod]")
	flag.Parse()

	if envFlag == "prod" {
		inputFileName = "./inputs/day1.prod"
	} else {
		inputFileName = "./inputs/day1.test"
	}

	data, err := os.ReadFile(inputFileName)

	if err != nil {
		log.Fatal(err)
	}

	dataString := string(data)

	var total int

	for _, line := range strings.Split(dataString, "\n") {
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

	println(total)
}
