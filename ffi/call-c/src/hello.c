#include <stdio.h>

void add_one(unsigned char arr[], int arr_len)
{
    int i;
    for (i = 0; i < arr_len; i++)
    {
        arr[i] += 1;
    }
}

void say_hello(const char *who)
{
    printf("hello, %s\n", who);
}