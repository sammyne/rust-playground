#include <stdio.h>
#include <stdlib.h>

typedef void (*callback_rs)();

void call_nullable_callback_from_rust(callback_rs callback)
{
    if (NULL == callback)
    {
        printf("call is nil\n");
        return;
    }

    printf("call is non-nil\n");
    callback();
}
