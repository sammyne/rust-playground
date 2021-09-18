#include <stdio.h>
#include <stdlib.h>

typedef struct {
  int x;
} Hello;

void add_one(unsigned char arr[], int arr_len)
{
    int i;
    for (i = 0; i < arr_len; i++)
    {
        arr[i] += 1;
    }
}

void free_hello(Hello *h) {
    free(h);
}

Hello* new_hello() {
    Hello *out = (Hello*)malloc(sizeof(Hello));
    out->x = 123;

    return out;
}

void print_hello(const Hello *h) {
    printf("hello.x = %d\n", h->x);
}

void say_hello(const char *who)
{
    printf("hello, %s\n", who);
}

