bits 32
global kloader_main

gdt_start:
    dq 0
    dq 0x00CF9A000000FFFF
    dq 0x00CF92000000FFFF
gdt_descriptor:
    dw $ - gdt_start - 1
    dd gdt_start

section .text
kloader_main:
    mov [mb2_info_ptr], ebx
    lgdt [gdt_descriptor]
    jmp 0x08:.after_cs_reload
.after_cs_reload:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov ebp, stack
    mov esp, stack

    cld
    cli
	extern kmain
    push dword [mb2_info_ptr]
    jmp kmain

section .bss
resb 16384
stack:

mb2_info_ptr:
resb 4