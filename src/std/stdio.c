#include <drv/vga.h>
#include <drv/e9.h>
#include <std/stdlib.h>

void putc(char c)
{
    vga_write_char(white, black, c);
}

void puts(const char* str)
{
    for (size i = 0; str[i]; i++)
        putc(str[i]);
}

void kputc(char c)
{
    UNUSED(c); // gcc is lowk drunk
    #ifdef VGA_DEBUG
    vga_write_char(light_gray, black, c);
    #endif

    #ifdef E9_DEBUG
    e9_write_char(c);
    #endif
}

void kputs(const char* str)
{
    for (size i = 0; str[i]; i++)
        kputc(str[i]);
}