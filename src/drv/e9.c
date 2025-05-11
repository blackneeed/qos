#include <types.h>
#include <drv/ioport.h>

void e9_write_char(u8 c) {
    io_outb(0xE9, c);
}

void e9_write_str(const char* s) {
    for (size i = 0; s[i] != '\0'; i++) {
        e9_write_char(s[i]);
    }
}