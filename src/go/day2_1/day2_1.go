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

func (g Game) HasRedLessThan(n int) bool {
	for _, hand := range g.hands {
		if hand.red > n {
			return false
		}
	}

	return true
}

func (g Game) HasGreenLessThan(n int) bool {
	for _, hand := range g.hands {
		if hand.green > n {
			return false
		}
	}

	return true
}

func (g Game) HasBlueLessThan(n int) bool {
	for _, hand := range g.hands {
		if hand.blue > n {
			return false
		}
	}

	return true
}

func main() {
	log.SetPrefix("Day 2 part 1")
	log.SetFlags(0)

	var envFlag string
	var inputFileName string

	flag.StringVar(&envFlag, "e", "prod", "Input environment to use [test, prod]")
	flag.StringVar(&envFlag, "environment", "prod", "Input environment to use [test, prod]")
	flag.Parse()

	if envFlag == "prod" {
		inputFileName = "./inputs/day2.prod"
	} else {
		inputFileName = "./inputs/day2_1.test"
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

		if game.HasRedLessThan(12) && game.HasGreenLessThan(13) && game.HasBlueLessThan(14) {
			total += game.id
		}
	}

	fmt.Println(total)
}
