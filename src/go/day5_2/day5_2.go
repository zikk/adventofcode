package main

import (
	"flag"
	"fmt"
	"os"
	"strconv"
	"strings"
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

	data, _ := os.ReadFile(inputFileName)
	location := -1

	lines := strings.Split(string(data), "\n")

	seeds := strings.Split(strings.TrimSpace(strings.Split(strings.TrimSpace(lines[0]), ":")[1]), " ")
	maps := strings.Split(strings.Join(lines[1:], "\n"), "\n\n")

	seedRanges := map[int]int{}

	for i := 0; i <= len(seeds)-1; i += 2 {
		key, _ := strconv.Atoi(seeds[i])
		value, _ := strconv.Atoi(seeds[i+1])
		seedRanges[key] = value
	}

	for key, seedRange := range seedRanges {
		for seed := key; key+seedRange >= seed; seed++ {
			seedLocation := seed

			for _, m := range maps {
				mappedMaps := strings.Split(strings.TrimSpace(strings.Split(strings.TrimSpace(m), ":")[1]), "\n")
				found := false

				for _, mappedMap := range mappedMaps {
					if found {
						break
					}

					for _, line := range strings.Split(mappedMap, "\n") {
						values := strings.Split(strings.TrimSpace(line), " ")
						d, _ := strconv.Atoi(values[0])
						l, _ := strconv.Atoi(values[1])
						step, _ := strconv.Atoi(values[2])
						s := step - 1

						if seedLocation >= l && seedLocation <= l+s {
							seedLocation = d + (seedLocation - l)
							found = true
						}
					}
				}
			}

			if location == -1 || seedLocation < location {
				location = seedLocation
			}
		}
	}

	fmt.Println(location)
}
