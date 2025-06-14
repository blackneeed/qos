#include <stdarg.h>

int vprintf(const char *restrict format, va_list ap);
int printf(const char *restrict format, ...);
int kvprintf(const char* restrict format, va_list ap);
int kprintf(const char* restrict format, ...);