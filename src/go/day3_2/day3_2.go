package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"os"
	"strconv"
)

type Point struct {
	x int
	y int
}

type Item struct {
	value       string
	isTouched   bool
	touchPoints []Point
}

type Matrix [][]Item

type Number struct {
	value       int
	touchPoints bool
}

func (m Matrix) checkIsTouched(y int, x int) (bool, []Point) {
	matrixLength := len(m[0])
	matrixHeight := len(m)
	maxXIndex := matrixLength - 1
	maxYIndex := matrixHeight - 1

	var isTouched bool
	var touchIndices []Point
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
			touchIndices = append(touchIndices, Point{x + 1, y})
		}
	}

	if hasLeftItem {
		leftItem := m[y][x-1]
		_, err := strconv.Atoi(leftItem.value)
		if err != nil && leftItem.value != "." {
			isTouched = true
			touchIndices = append(touchIndices, Point{x - 1, y})
		}
	}

	if hasTopItem {
		topItem := m[y-1][x]
		_, err := strconv.Atoi(topItem.value)
		if err != nil && topItem.value != "." {
			isTouched = true
			touchIndices = append(touchIndices, Point{x, y - 1})
		}
	}

	if hasBottomItem {
		bottomItem := m[y+1][x]
		_, err := strconv.Atoi(bottomItem.value)
		if err != nil && bottomItem.value != "." {
			isTouched = true
			touchIndices = append(touchIndices, Point{x, y + 1})
		}
	}

	if hasTopRightItem {
		topRightItem := m[y-1][x+1]
		_, err := strconv.Atoi(topRightItem.value)
		if err != nil && topRightItem.value != "." {
			isTouched = true
			touchIndices = append(touchIndices, Point{x + 1, y - 1})
		}
	}

	if hasTopLeftItem {
		topLeftItem := m[y-1][x-1]
		_, err := strconv.Atoi(topLeftItem.value)
		if err != nil && topLeftItem.value != "." {
			isTouched = true
			touchIndices = append(touchIndices, Point{x - 1, y - 1})
		}
	}

	if hasBottomRightItem {
		bottomRightItem := m[y+1][x+1]
		_, err := strconv.Atoi(bottomRightItem.value)
		if err != nil && bottomRightItem.value != "." {
			isTouched = true
			touchIndices = append(touchIndices, Point{x + 1, y + 1})
		}
	}

	if hasBottomLeftItem {
		bottomLeftItem := m[y+1][x-1]
		_, err := strconv.Atoi(bottomLeftItem.value)
		if err != nil && bottomLeftItem.value != "." {
			isTouched = true
			touchIndices = append(touchIndices, Point{x - 1, y + 1})
		}
	}

	return isTouched, touchIndices
}

func main() {
	log.SetPrefix("Day 3 part 2")
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
			isItemTouched, indices := matrix.checkIsTouched(y, x)

			if err == nil && isItemTouched {
				item.isTouched = true
				item.touchPoints = indices
			}
		}
	}

	numbers := []Item{}
	var touchedNumbers [][]Item
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
					touchedNumbers = append(touchedNumbers, numbers)
				}

				numbers = []Item{}
			}
		}
	}

	for y, line := range matrix {
		for x, item := range line {
			if item.value == "*" {
				var numbersTouchedByAsterisk [][]Item

				for _, number := range touchedNumbers {
					var isTouched bool
					for _, digit := range number {
						for _, p := range digit.touchPoints {
							if p.x == x && p.y == y {
								isTouched = true
							}
						}
					}

					if isTouched {
						numbersTouchedByAsterisk = append(numbersTouchedByAsterisk, number)
					}
				}

				if len(numbersTouchedByAsterisk) == 2 {
					gearRatio := 1

					for _, number := range numbersTouchedByAsterisk {
						var numberStr string

						for _, digit := range number {
							numberStr += digit.value
						}

						n, _ := strconv.Atoi(numberStr)
						gearRatio *= n
					}

					total += gearRatio
				}
			}
		}
	}

	fmt.Println(total)
}
