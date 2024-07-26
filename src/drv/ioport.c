#include <types.h>

inline void io_outb(u16 port, u8 val)
{
    __asm__ volatile ("outb %b0, %w1" : : "a"(val), "Nd"(port) : "memory");
}

inline u8 io_inb(u16 port)
{
    u8 ret;
    __asm__ volatile ("inb %w1, %b0" : "=a"(ret) : "Nd"(port) : "memory");
    return ret;
}

inline void io_wait()
{
    io_outb(0x80, 0);
}