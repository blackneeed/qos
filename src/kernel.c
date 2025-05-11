#include <drv/vga.h>
#include <tables/idt/idt.h>
#include <drv/pic.h>

vga_info info_;
vga_info* info = &info_;

void quickos_kernel_loop()
{
    
}

void quickos_kernel_entry() 
{
    vga_pos pos = {.x = 0, .y = 0};
    vga_size size = {.w = 80, .h = 25};
    vga_color color = {.fg = light_cyan, .bg = black};
    vga_initialize(info, (char*)0xb8000, pos, size);
    vga_clear(info, color);
    vga_write_str_line(info, color, "Hello, world!");
    pic_remap(0x20, 0x28);
    
    for (int i = 0; i < 8; i++) {
        pic_mask(i);
        pic_mask(i + 8);
    }

    pic_unmask(1);
    idt_init();

    for (;;) __asm__ volatile ("hlt");
    
    //while (1) __asm__ volatile ("hlt");
}