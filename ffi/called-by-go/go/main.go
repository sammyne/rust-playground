package main

//#cgo LDFLAGS: -L${SRCDIR}/../build/target/debug -lhelloworld
//#include <stdlib.h>
//typedef struct HelloWorld hello_world_t;
//extern void hello_world(const char* who);
//extern hello_world_t *hello_world_new(const char *who);
//extern void hello_world_free(hello_world_t *);
//extern void hello_world_say(const hello_world_t *);
import "C"
import "unsafe"

func main() {
	who := C.CString("go")
	defer C.free(unsafe.Pointer(who))

	C.hello_world(who)

	hw := C.hello_world_new(who)
	defer C.hello_world_free(hw)

	C.hello_world_say(hw)
}
