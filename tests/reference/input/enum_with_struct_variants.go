package main

import (
	"encoding/gob"
	"os"
)

type V1 struct {
	Foo bool
}

type V2 struct {
	Bar int64
	Baz uint64
}

type V3 struct {
	Quux string
}

type Enum struct {
	V1 *V1
	V2 *V2
	V3 *V3
}

func main() {
	var enc = gob.NewEncoder(os.Stdout)
	enc.Encode(Enum{V2: &V2{Bar: 42, Baz: 1234}})
}
