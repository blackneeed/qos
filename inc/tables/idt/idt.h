#ifndef IDT_H
#define IDT_H
#include <types.h>

typedef struct idt_entry_t
{
    u16 isr_low;
    u16 kernel_cs;
    u8 reserved;
    u8 attributes;
    u16 isr_high;
} __attribute__((packed)) idt_entry_t;

typedef struct idtr_t
{
    u16 limit;
    u32 base;
} __attribute__((packed)) idtr_t;

void idt_set_descriptor(u16 vector, void* isr, u8 flags);
void idt_init();
#endif