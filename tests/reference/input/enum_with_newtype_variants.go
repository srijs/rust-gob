package main

import (
	"encoding/gob"
	"os"
)

type Enum struct {
	Var1 bool
	Var2 int64
	Var3 string
}

func main() {
	var enc = gob.NewEncoder(os.Stdout)
	enc.Encode(Enum{Var2: 42})
}
