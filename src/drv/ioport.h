#pragma once
#include <types.h>

void io_outb(u16 port, u8 val);
u8 io_inb(u16 port);
void io_wait();