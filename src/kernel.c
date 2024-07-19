#include <drv/vga.h>

void quickos_kernel_entry() {
    vga_info info;
    vga_pos pos = {.x = 0, .y = 0};
    vga_size size = {.w = 80, .h = 25};
    vga_color color = {.fg = white, .bg = black};
    vga_initialize(&info, (char*)0xb8000, pos, size);
    vga_write_str_line(&info, color, "Hello, world!");
}