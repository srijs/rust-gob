package main

import (
	"encoding/gob"
	"os"
)

type BoolStruct struct {
	V bool
}

func main() {
	var enc = gob.NewEncoder(os.Stdout)
	enc.Encode(BoolStruct{V: true})
}
