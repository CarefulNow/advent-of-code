package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
    file, err := os.Open("input.txt")
    if err != nil {
	log.Fatalf("Unable to read file: %v", err)
    }
    inputScanner := bufio.NewScanner(file)
    totalPaperNeeded := 0
    totalRibbonNeeded := 0
    for inputScanner.Scan() {
        box := strings.Split(inputScanner.Text(), "x")
	height, _ := strconv.Atoi(box[0])
	width, _ := strconv.Atoi(box[1])
	length, _ := strconv.Atoi(box[2])
        paperNeeded := calculatePaperNeededForBox(
	    height,
	    width,
	    length,
	)
	totalPaperNeeded += paperNeeded
        ribbonNeeded := calculateRibbonNeededForBox(
            height,
            width,
            length,
        )
        totalRibbonNeeded += ribbonNeeded
    }
    fmt.Printf("Total Wrapping Paper Needed %d\n", totalPaperNeeded)
    fmt.Printf("Total Ribbon Needed %d\n", totalRibbonNeeded)
}

func calculatePaperNeededForBox(
    height int,
    width int,
    length int,
) int {
    side1 := length * width * 2
    side2 := width * height * 2
    side3 := height * length * 2

    paperNeeded := side1 + side2 + side3
    if side1 <= side2 && side1 <= side3 {
        paperNeeded += side1 / 2
    } else if side2 <= side1 && side2 <= side3 {
	paperNeeded += side2 / 2
    } else if side3 <= side1 && side3 <= side2 {
        paperNeeded += side3 / 2
    }
    
    return paperNeeded
}

func calculateRibbonNeededForBox(
    height int,
    width int,
    length int,
) int {
    bowRibbon := width * height * length
    perimeterRibbon := (width * 2) + (height * 2) + (length * 2)
    
    maxLength := 0
    if (height >= maxLength) {
        maxLength = height;
    }
    if (width >= maxLength) {
        maxLength = width;
    }
    if (length >= maxLength) {
        maxLength = length;
    }
    perimeterRibbon -= maxLength * 2;
    return bowRibbon + perimeterRibbon;
}
