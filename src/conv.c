#include <types.h>
#include <string.h>

char buf[32];

const char* u32_to_str(u32 value)
{
    size len;
    do
    {
      const char digit = (char)(value % 10);
      buf[len++] = (char)('0' + digit);
      value /= 10;
    } while (value && (len < 32));
    return &buf[0];
}

u32 cPow(u32 base, u32 power)
{
    u32 output = 1;
    for (unsigned int i = 0; i < power; i++) {
        output *= base;
    }
    return output;
}

u32 str_to_u32(const char* string)
{
    u32 value = 0;
    size length = strlen(string);

    for (size i = 0; i < length; i++)
    {
        u32 placeValue = cPow(10, length - i - 1);
        value += (string[i] - '0') * placeValue;
    }

    return (u32)value;
}