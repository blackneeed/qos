#define E9_DEBUG
#include <tables/idt/idt.h>
#include <drv/pic.h>
#include <drv/ioport.h>
#include <std/stdio.h>

void quickos_kernel_loop()
{
    __asm__ volatile ("hlt");
}

void quickos_kernel_entry() 
{
    puts("Welcome from qos!\r\n");
    pic_remap(PIC_MASTER_START, PIC_SLAVE_START);

    for (int i = 0; i < 8; i++) {
        pic_mask(i);
        pic_mask(i + 8);
    }

    pic_unmask(1);
    idt_init();

    for (;;) quickos_kernel_loop();
}