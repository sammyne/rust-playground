#include <cstdio>

#include "helloworld.h"

int main()
{
    int input = 4;
    int output = double_input(input);

    printf("%d * 2 = %d\n", input, output);

    auto hw = hello_world_new("sammy");

    hello_world_say(hw);

    hello_world_free(hw);

    return 0;
}