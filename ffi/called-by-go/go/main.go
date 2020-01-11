package main

//#cgo LDFLAGS: -L${SRCDIR}/../build/target/debug -lhelloworld
//#include "hello.h"
import "C"
import "fmt"

func main() {
	x := C.hello_world_new(123)
	defer C.hello_world_free(x)

	fmt.Println("x:", x)
}
