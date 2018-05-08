package main

import (
	"encoding/gob"
	"os"
)

func main() {
	var enc = gob.NewEncoder(os.Stdout)
	enc.Encode(false)
	enc.Encode(uint(0))
	enc.Encode(int(0))
	enc.Encode(0.0)
	enc.Encode("")
	enc.Encode([]byte{})
	enc.Encode([]bool{})
}
