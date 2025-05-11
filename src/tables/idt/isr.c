#include <drv/vga.h>
#include <drv/pic.h>
#include <kernel_header.h>
#include <conv.h>

const char* exception_strings[] = {
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

void isr_handler(u32 int_no, u32 err_code)
{
    vga_color error_color = {.fg = light_red, .bg = black};
    vga_color ok_color = {.fg = light_cyan, .bg = black};
    if (int_no < 32)
    {
        vga_write_str_line(info, error_color, exception_strings[int_no]);
        __asm__ volatile ("cli; hlt");
    } else 
    {
        if (int_no == 33)
        {
            vga_write_str_line(info, ok_color, "IRQ1 called.");
            pic_send_eoi(1);
            vga_write_str_line(info, ok_color, "EOI sent.");
        } else vga_write_str_line(info, ok_color, "ISR called.");
    }
}