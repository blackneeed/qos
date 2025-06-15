#include <nanoprintf.h>
#include <stdarg.h>
#include <std/stdlib.h>
#include <drv/e9.h>
#include <cli.h>

void cli_putc_npf_wrapper(int c, void* ctx)
{
    UNUSED(ctx);
    cli_write_char(c);
}

void e9_putc_npf_wrapper(int c, void* ctx)
{
    UNUSED(ctx);
    e9_write_char(c);
}

int dvprintf(const char *restrict format, va_list ap)
{
    return npf_vpprintf(e9_putc_npf_wrapper, NULL, format, ap);
}

int vprintf(const char *restrict format, va_list ap)
{
    return npf_vpprintf(cli_putc_npf_wrapper, NULL, format, ap);
}

int printf(const char *restrict format, ...)
{
    va_list args;
    va_start(args, format);
    int rv = vprintf(format, args);
    va_end(args);
    return rv;
}

int dprintf(const char *restrict format, ...)
{
    va_list args;
    va_start(args, format);
    int rv = dvprintf(format, args);
    va_end(args);
    return rv;
}

int kvprintf(const char* restrict format, va_list ap)
{
    dvprintf(format, ap);
    cli_set_color(0x808080);
    int rv = vprintf(format, ap);
    cli_restore_color();
    return rv;
}

int kprintf(const char* restrict format, ...)
{
    va_list args;
    va_start(args, format);
    int rv = kvprintf(format, args);
    va_end(args);
    return rv;
}