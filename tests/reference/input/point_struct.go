package main

import (
	"encoding/gob"
	"os"
)

type Point struct {
	X int64
	Y int64
}

func main() {
	var enc = gob.NewEncoder(os.Stdout)
	enc.Encode(Point{X: 22, Y: 33})
}
