bits 32
global quickos_loader_entry
extern quickos_kernel_entry

section .multiboot
align 4
dd 0x1BADB002
dd ((1 << 1) | (1 << 0))
dd -(0x1BADB002 + ((1 << 1) | (1 << 0)))

section .text
quickos_loader_entry:
    mov ebp, stack
    mov esp, stack

    call quickos_kernel_entry

    cli
    .halt:
    hlt
    jmp .halt

section .bss
resb 4096
stack: