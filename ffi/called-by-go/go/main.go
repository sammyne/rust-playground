package main

//#cgo LDFLAGS: -L${SRCDIR}/../build/target/debug -lhelloworld
//#include <stdlib.h>
//#include "hello.h"
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	who := C.CString("go")
	defer C.free(unsafe.Pointer(who))

	C.hello_world(who)

	hw := C.hello_world_new(who)
	defer C.hello_world_free(hw)

	C.hello_world_say(hw)

	cstr := C.cstring_new()
	defer C.cstring_free(cstr)

	fmt.Println("cstring =", C.GoString(cstr))

	hi := C.hi_new()
	fmt.Println("hi:", hi)
}
