#include <drv/vga.h>

static vga_info* info;
void quickos_kernel_entry() 
{
    vga_pos pos = {.x = 0, .y = 0};
    vga_size size = {.w = 80, .h = 25};
    vga_color color = {.fg = light_cyan, .bg = black};
    vga_initialize(info, (char*)0xb8000, pos, size);
    vga_clear(info, color);
    vga_write_str_line(info, color, "Hello, world!");
}