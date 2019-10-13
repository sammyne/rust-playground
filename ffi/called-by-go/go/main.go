package main

//#cgo LDFLAGS: -L${SRCDIR}/../build/target/debug -lhelloworld
//#include <stdlib.h>
//extern void hello_world(const char* who);
import "C"
import "unsafe"

func main() {
	who := C.CString("go")
	defer C.free(unsafe.Pointer(who))

	C.hello_world(who)
}
