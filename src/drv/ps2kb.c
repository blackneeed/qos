#include <drv/ioport.h>

void ps2_keyboard_interrupt()
{
    io_inb(0x60);
    io_wait();
}