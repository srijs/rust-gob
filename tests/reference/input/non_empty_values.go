package main

import (
	"encoding/gob"
	"os"
)

func main() {
	var enc = gob.NewEncoder(os.Stdout)
	enc.Encode(true)
	enc.Encode(uint(42))
	enc.Encode(int(42))
	enc.Encode(42.0)
	enc.Encode("foo")
	enc.Encode([]byte{0x01, 0x02})
	enc.Encode([]bool{true, false})
}
