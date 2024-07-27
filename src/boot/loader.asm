bits 32
global quickos_loader_entry
extern quickos_kernel_entry

gdt_start:
    dq 0
    dq 0x00CF9A000000FFFF
    dq 0x00CF92000000FFFF

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

section .multiboot
align 4
dd 0x1BADB002
dd ((1 << 1) | (1 << 0))
dd -(0x1BADB002 + ((1 << 1) | (1 << 0)))

section .text
quickos_loader_entry:
    lgdt [gdt_descriptor]
    jmp 0x08:quickos_loader_after_cs_reload
quickos_loader_after_cs_reload:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov ebp, stack
    mov esp, stack

    cld
    cli
    jmp quickos_kernel_entry

section .bss
resb 4096
stack: