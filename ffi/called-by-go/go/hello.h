#pragma once

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct HelloWorld hello_world_t;

    char *cstring_new();
    void cstring_free(char *str);

    void hello_world(const char *who);
    hello_world_t *hello_world_new(const char *who);
    void hello_world_free(hello_world_t *);
    void hello_world_say(const hello_world_t *);

#ifdef __cplusplus
}
#endif