#include <types.h>
#include <stdint.h>
#include "idt.h"

#define IDT_MAX_DESCRIPTORS 256

static idtr_t idtr;

__attribute__((aligned(0x10)))
static idt_entry_t idt[IDT_MAX_DESCRIPTORS];

static int vectors[IDT_MAX_DESCRIPTORS];

extern void* isr_stub_table[];

void idt_set_descriptor(u8 vector, void* isr, u8 flags)
{
    idt_entry_t* descriptor = &idt[vector];

    descriptor->isr_low = (u32)isr & 0xFFFF;
    descriptor->kernel_cs = 0x08;
    descriptor->attributes = flags;
    descriptor->isr_high = (u32)isr >> 16;
    descriptor->reserved = 0;
}

void idt_init()
{
    idtr.base = (uintptr_t)&idt[0];
    idtr.limit = (u16)(sizeof(idt_entry_t) * IDT_MAX_DESCRIPTORS - 1);

    for (u8 vector = 0; vector < IDT_MAX_DESCRIPTORS; vector++) {
        idt_set_descriptor(vector, isr_stub_table[vector], 0x8E);
    }

    __asm__ volatile("lidt %0" : : "m"(idtr));

    __asm__ volatile("sti");
}
