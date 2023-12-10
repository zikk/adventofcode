package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"os"
	"strconv"
)

type Item struct {
	value     string
	isTouched bool
}

type Matrix [][]Item

func (m Matrix) checkIsTouched(y int, x int) bool {
	matrixLength := len(m[0])
	matrixHeight := len(m)
	maxXIndex := matrixLength - 1
	maxYIndex := matrixHeight - 1

	var isTouched bool
	hasRightItem := x+1 <= maxXIndex
	hasLeftItem := x-1 >= 0
	hasTopItem := y-1 >= 0
	hasBottomItem := y+1 <= maxYIndex
	hasTopRightItem := hasRightItem && hasTopItem
	hasTopLeftItem := hasLeftItem && hasTopItem
	hasBottomRightItem := hasBottomItem && hasRightItem
	hasBottomLeftItem := hasBottomItem && hasLeftItem

	if hasRightItem {
		rightItem := m[y][x+1]
		_, err := strconv.Atoi(rightItem.value)
		if err != nil && rightItem.value != "." {
			isTouched = true
		}
	}

	if hasLeftItem {
		leftItem := m[y][x-1]
		_, err := strconv.Atoi(leftItem.value)
		if err != nil && leftItem.value != "." {
			isTouched = true
		}
	}

	if hasTopItem {
		topItem := m[y-1][x]
		_, err := strconv.Atoi(topItem.value)
		if err != nil && topItem.value != "." {
			isTouched = true
		}
	}

	if hasBottomItem {
		bottomItem := m[y+1][x]
		_, err := strconv.Atoi(bottomItem.value)
		if err != nil && bottomItem.value != "." {
			isTouched = true
		}
	}

	if hasTopRightItem {
		topRightItem := m[y-1][x+1]
		_, err := strconv.Atoi(topRightItem.value)
		if err != nil && topRightItem.value != "." {
			isTouched = true
		}
	}

	if hasTopLeftItem {
		topLeftItem := m[y-1][x-1]
		_, err := strconv.Atoi(topLeftItem.value)
		if err != nil && topLeftItem.value != "." {
			isTouched = true
		}
	}

	if hasBottomRightItem {
		bottomRightItem := m[y+1][x+1]
		_, err := strconv.Atoi(bottomRightItem.value)
		if err != nil && bottomRightItem.value != "." {
			isTouched = true
		}
	}

	if hasBottomLeftItem {
		bottomLeftItem := m[y+1][x-1]
		_, err := strconv.Atoi(bottomLeftItem.value)
		if err != nil && bottomLeftItem.value != "." {
			isTouched = true
		}
	}

	return isTouched
}

func main() {
	log.SetPrefix("Day 2 part 2")
	log.SetFlags(0)

	var envFlag string
	var inputFileName string

	flag.StringVar(&envFlag, "e", "prod", "Input environment to use [test, prod]")
	flag.StringVar(&envFlag, "environment", "prod", "Input environment to use [test, prod]")
	flag.Parse()

	if envFlag == "prod" {
		inputFileName = "./inputs/day3.prod"
	} else {
		inputFileName = "./inputs/day3.test"
	}

	file, err := os.Open(inputFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	fileScanner := bufio.NewScanner(file)
	total := 0
	lineIndex := -1
	var matrix Matrix

	for fileScanner.Scan() {
		lineIndex += 1
		line := fileScanner.Text()
		matrix = append(matrix, []Item{})

		for _, char := range line {
			matrix[lineIndex] = append(matrix[lineIndex], Item{value: string(char)})
		}
	}

	for y := range matrix {
		line := &matrix[y]

		for x := range *line {
			item := &matrix[y][x]
			_, err := strconv.Atoi(string(item.value))

			if err == nil && matrix.checkIsTouched(y, x) {
				item.isTouched = true
			}
		}
	}

	numbers := []Item{}
	for _, line := range matrix {

		for _, item := range line {
			if _, err := strconv.Atoi(string(item.value)); err == nil {
				numbers = append(numbers, item)
			} else {
				var isTouched bool
				var number string

				if len(numbers) == 0 {
					continue
				}

				for _, n := range numbers {
					number += n.value

					if n.isTouched {
						isTouched = true
					}
				}

				if isTouched {
					num, _ := strconv.Atoi(number)
					total += num
				}

				numbers = []Item{}
			}
		}
	}

	fmt.Println(total)
}
