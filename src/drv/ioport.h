#pragma once
#include <types.h>

static inline void io_outb(u16 port, u8 val);
static inline u8 io_inb(u16 port);
static inline void io_wait();