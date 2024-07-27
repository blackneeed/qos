#include <drv/vga.h>
#include <kernel_header.h>

const char* exception_strings[32] = {
    "(#DE) Division Error",
    "(#DB) Debug",
    "(#--) Non-maskable Interrupt",
    "(#BP) Breakpoint",
    "(#OF) Overflow",
    "(#BR) Bound Range Exceeded",
    "(#UD) Invalid Opcode",
    "(#NM) Device Not Available",
    "(#DF) Double Fault",
    "(#--) Coprocessor Segment Overrun",
    "(#TS) Invalid TSS",
    "(#NP) Segment Not Present",
    "(#SS) Stack Segment Fault",
    "(#GP) General Protection Fault",
    "(#PF) Page Fault",
    "(#--) Reserved",
    "(#MF) x87 Floating-Point Exception",
    "(#AC) Alignment Check",
    "(#MC) Machine Check",
    "(#XM) SIMD Floating-Point Exception",
    "(#VE) Virtualization Exception",
    "(#CP) Control Protection Exception",
    "", "", "", "", "", "",
    "(#HV) Hypervisor Injection Exception",
    "(#VC) VMM Communication Exception",
    "(#SX) Security Exception",
    "(#--) Reserved"
};

void isr_handler(int int_no)
{
    vga_color error_color = {.fg = light_red, .bg = black};
    vga_color ok_color = {.fg = light_cyan, .bg = black};
    if (int_no < 32)
    {
        vga_write_str_line(info, error_color, exception_strings[int_no]);
        for (;;) __asm__("hlt");
    } else 
    {
        vga_write_str_line(info, ok_color, "ISR called.");
    }
}