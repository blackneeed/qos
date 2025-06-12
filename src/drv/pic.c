#include <types.h>
#include <drv/ioport.h>
#include <drv/pic.h>

void pic_send_slave_eoi()
{
    io_outb(PIC2_COMMAND, PIC_EOI);
    io_wait();
    pic_send_master_eoi();
}

void pic_send_master_eoi()
{
    io_outb(PIC1_COMMAND, PIC_EOI);
    io_wait();
}

void pic_remap(int off1, int off2) {
	u8 a1 = io_inb(PIC1_DATA);
	u8 a2 = io_inb(PIC2_DATA);
	
	io_outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);  // starts the initialization sequence (in cascade mode)
	io_wait();
	io_outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
	io_wait();
	io_outb(PIC1_DATA, off1);                 // ICW2: Master PIC vector offset
	io_wait();
	io_outb(PIC2_DATA, off2);                 // ICW2: Slave PIC vector offset
	io_wait();
	io_outb(PIC1_DATA, 4);                       // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
	io_wait();
	io_outb(PIC2_DATA, 2);                       // ICW3: tell Slave PIC its cascade identity (0000 0010)
	io_wait();
	
	io_outb(PIC1_DATA, ICW4_8086);               // ICW4: have the PICs use 8086 mode (and not 8080 mode)
	io_wait();
	io_outb(PIC2_DATA, ICW4_8086);
	io_wait();
	
	io_outb(PIC1_DATA, a1);
	io_outb(PIC2_DATA, a2);
}

void pic_mask(u8 irq) {
    u16 port;
    u8 value;

    if(irq < 8) {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        irq -= 8;
    }
    value = io_inb(port) | (1 << irq);
    io_outb(port, value);        
}

void pic_unmask(u8 irq) {
    u16 port;
    u8 value;

    if(irq < 8) {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        irq -= 8;
    }
    value = io_inb(port) & ~(1 << irq);
    io_outb(port, value);        
}