#include <cstdio>
#include <cstdlib>

extern "C"
{
    typedef int (*callback_rs)(int);

    typedef struct
    {
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

    int call_callback_from_rust(int v, callback_rs callback)
    {
        return callback(v);
    }

    void free_hello(Hello *h)
    {
        free(h);
    }

    Hello *new_hello()
    {
        Hello *out = (Hello *)malloc(sizeof(Hello));
        out->x = 123;

        return out;
    }

    void print_hello(const Hello *h)
    {
        printf("hello.x = %d\n", h->x);
    }

    void print_string_slice(int ell, const char **s)
    {
        for (auto i = 0; i < ell; ++i)
        {
            printf("s[%d]=%s\n", i, s[i]);
        }
    }

    void say_hello(const char *who)
    {
        printf("hello, %s\n", who);
    }
}