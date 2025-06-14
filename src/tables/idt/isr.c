#include <drv/pic.h>
#include <std/stdlib.h>
#include <std/stdio.h>
#include <tables/idt/irq.h>
#include <drv/e9.h>
#include <cli.h>

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
    UNUSED(err_code);
    if (int_no < 32)
    {
        kprintf("%s\r\n", exception_strings[int_no]);
        __asm__ volatile ("cli; hlt");
    } else if (int_no == PIC_MASTER_START + 0)
    {
        irq0_handler();
        pic_send_master_eoi();
    } else if (int_no == PIC_MASTER_START + 1)
    { 
        irq1_handler();
        pic_send_master_eoi();   
    } else if (int_no == PIC_MASTER_START + 2)
    { 
        irq2_handler();
        pic_send_master_eoi();   
    } else if (int_no == PIC_MASTER_START + 3)
    { 
        irq3_handler();
        pic_send_master_eoi();   
    } else if (int_no == PIC_MASTER_START + 4)
    { 
        irq4_handler();
        pic_send_master_eoi();   
    } else if (int_no == PIC_MASTER_START + 5)
    { 
        irq5_handler();
        pic_send_master_eoi();   
    } else if (int_no == PIC_MASTER_START + 6)
    { 
        irq6_handler();
        pic_send_master_eoi();   
    } else if (int_no == PIC_MASTER_START + 7)
    { 
        irq7_handler();
        pic_send_master_eoi();   
    } else if (int_no == PIC_SLAVE_START + 0) {
        irq8_handler();
        pic_send_slave_eoi();
    } else if (int_no == PIC_SLAVE_START + 1) {
        irq9_handler();
        pic_send_slave_eoi();
    } else if (int_no == PIC_SLAVE_START + 2) {
        irq10_handler();
        pic_send_slave_eoi();
    } else if (int_no == PIC_SLAVE_START + 3) {
        irq11_handler();
        pic_send_slave_eoi();
    } else if (int_no == PIC_SLAVE_START + 4) {
        irq12_handler();
        pic_send_slave_eoi();
    } else if (int_no == PIC_SLAVE_START + 5) {
        irq13_handler();
        pic_send_slave_eoi();
    } else if (int_no == PIC_SLAVE_START + 6) {
        irq14_handler();
        pic_send_slave_eoi();
    } else if (int_no == PIC_SLAVE_START + 7) {
        irq15_handler();
        pic_send_slave_eoi();
    } else {
        kprintf("Unhandled ISR.\r\n");
    }
}