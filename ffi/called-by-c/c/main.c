#include <stdint.h>
#include <stdio.h>

#include "helloworld.h"

// HelloWorld will be mapped to that of rust's HelloWorld automatically.
// HelloWorld can be any valid name you want.
//typedef struct HelloWorld hello_world_t;
//
//extern hello_world_t *hello_world_new(const char *who);
//extern void hello_world_free(hello_world_t *);
//extern void hello_world_say(const hello_world_t *);

//extern int32_t double_input(int32_t input);

int main()
{
    int input = 4;
    int output = double_input(input);

    printf("%d * 2 = %d\n", input, output);

    hello_world_t *hw = hello_world_new("sammy");

    hello_world_say(hw);

    hello_world_free(hw);

    return 0;
}