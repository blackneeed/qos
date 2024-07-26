#include <drv/vga.h>
#include <kernel_header.h>

void exception_handler()
{
    vga_color color = {.fg = light_red, .bg = black};
    vga_write_str_line(info, color, "An exception occured.");
    __asm__("cli");
    for (;;) __asm__("hlt");
}