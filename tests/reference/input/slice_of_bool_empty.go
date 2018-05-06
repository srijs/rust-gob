package main

import (
	"encoding/gob"
	"os"
)

func main() {
	var enc = gob.NewEncoder(os.Stdout)
	enc.Encode([]bool{})
}
