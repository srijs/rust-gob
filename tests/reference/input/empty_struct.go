package main

import (
	"encoding/gob"
	"os"
)

type EmptyStruct struct {
}

func main() {
	var enc = gob.NewEncoder(os.Stdout)
	enc.Encode(EmptyStruct{})
}
