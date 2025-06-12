#include <drv/ioport.h>
#include <drv/ps2kb.h>
#include <cli.h>

int ps2_set_scancode_set(u8 set)
{
    if (set < 1 || set > 3) return 1;
    io_wait();
    io_outb(0x60, 0xF0);
    io_wait();
    io_outb(0x60, set);
    io_wait();

    if (io_inb(0x60) == 0xFA) return 0;
    else return 1;
}

void ps2_keyboard_interrupt()
{
    io_wait();
    u8 scancode = io_inb(0x60);
    if (scancode < 0x3A)
        cli_put_char(ps2kb_scancode_set2_map[scancode]);
    io_wait();
}