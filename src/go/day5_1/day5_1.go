package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	var envFlag string
	var inputFileName string

	flag.StringVar(&envFlag, "e", "prod", "Input environment to use [test, prod]")
	flag.StringVar(&envFlag, "environment", "prod", "Input environment to use [test, prod]")
	flag.Parse()

	if envFlag == "prod" {
		inputFileName = "./inputs/day5.in"
	} else {
		inputFileName = "./inputs/day5.test.in"
	}

	data, _ := os.OpenFile(inputFileName)
	total := 0

	for fileScanner.Scan() {
		//
	}

	fmt.Println(total)
}
