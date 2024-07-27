#include <types.h>

size strlen(const char* str)
{
    size res;
    for (res = 0; str[res]; res++)
    {}
    return res;
}