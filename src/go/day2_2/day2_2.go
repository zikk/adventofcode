package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type GameHand struct {
	red   int
	green int
	blue  int
}

type Game struct {
	id    int
	hands []GameHand
}

func (g Game) MaxRed() int {
	max := 0

	for _, hand := range g.hands {
		if hand.red > max {
			max = hand.red
		}
	}

	return max
}

func (g Game) MaxGreen() int {
	max := 0

	for _, hand := range g.hands {
		if hand.green > max {
			max = hand.green
		}
	}

	return max
}

func (g Game) MaxBlue() int {
	max := 0

	for _, hand := range g.hands {
		if hand.blue > max {
			max = hand.blue
		}
	}

	return max
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
		inputFileName = "./inputs/day2.prod"
	} else {
		inputFileName = "./inputs/day2.test"
	}

	file, err := os.Open(inputFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	fileScanner := bufio.NewScanner(file)
	total := 0

	for fileScanner.Scan() {
		line := fileScanner.Text()
		parts := strings.Split(line, ":")
		gameIdData := parts[0]
		gameHands := strings.Split(parts[1], ";")
		digitRe := regexp.MustCompile("\\d+")
		gameId, _ := strconv.Atoi(string(digitRe.FindString(gameIdData)))

		gameHandRe := regexp.MustCompile("\\d+ (red|green|blue)")
		var hands []GameHand

		for _, gameHand := range gameHands {
			colors := gameHandRe.FindAllString(gameHand, -1)
			var hand GameHand

			for _, colorInfo := range colors {
				split := strings.Split(colorInfo, " ")
				value, _ := strconv.Atoi(split[0])
				switch split[1] {
				case "red":
					hand.red = value
				case "green":
					hand.green = value
				case "blue":
					hand.blue = value
				}
			}

			hands = append(hands, hand)
		}

		game := Game{id: gameId, hands: hands}
		total += game.MaxRed() * game.MaxGreen() * game.MaxBlue()
	}

	fmt.Println(total)
}
