package main

import (
    "fmt"
    "os"
    "log"
)

func main() {
    body, err := os.ReadFile("input.txt")
    if err != nil {
        log.Fatalf("Unable to read file: %v", err)
    }

    floorNumber := 0
    firstTimeInBasement := false
    for i, r := range body {
        c := string(r)
        if (c == "(") {
            floorNumber++
        } else if (c == ")") {
            floorNumber--
        }
        if (floorNumber < 0 && !firstTimeInBasement) {
            firstTimeInBasement = true
            fmt.Printf("Position in the basement %d\n", i + 1)
        }
    }
    fmt.Printf("Floor Number: %d", floorNumber)
}

