#pragma once
#include <types.h>

#define PIC1_BASE 0x0020
#define PIC2_BASE 0x00A0
#define PIC1_COMMAND PIC1_BASE
#define PIC1_DATA (PIC1_BASE + 1)
#define PIC2_COMMAND PIC2_BASE
#define PIC2_DATA (PIC2_BASE + 1)

#define PIC_EOI 0x20

#define ICW1_ICW4 0x01
#define ICW1_SINGLE 0x02
#define ICW1_INTERVAL4 0x04
#define ICW1_LEVEL 0x08
#define ICW1_INIT 0x10

#define ICW4_8086 0x01
#define ICW4_AUTO 0x02
#define ICW4_BUF_SLAVE 0x08
#define ICW4_BUF_MASTER	0x0C
#define ICW4_SFNM 0x10

void pic_send_eoi(u8 irq);
void pic_remap(int off1, int off2); // 0 8