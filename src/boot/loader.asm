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
align 8
multiboot2_header_start:
dd 0xE85250D6
dd 0
dd multiboot2_header_end - multiboot2_header_start
dd -(0xE85250D6 + 0 + (multiboot2_header_end - multiboot2_header_start))

align 8
framebuffer_tag_start:  
dw 5 ; fb tag type
dw 1 ; optional ig since we have a vga fallback to print error
dd framebuffer_tag_end - framebuffer_tag_start
dd 0
dd 0
dd 0
framebuffer_tag_end:

align 8
mmap_tag_start:
dw 6 ; mmap tag type
dw 0
dd mmap_tag_end - mmap_tag_start
mmap_tag_end:

end_tag_start:
align 8
dw 0
dw 0
dd end_tag_end - end_tag_start
end_tag_end:
multiboot2_header_end:

section .text
quickos_loader_entry:
    cmp eax, 0x36D76289
    je quickos_loader_after_bootloader_magic_check
    cli
    .halt:
    hlt
    jmp .halt
quickos_loader_after_bootloader_magic_check:
    mov [mb2_info_ptr], ebx
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
    push dword [mb2_info_ptr]
    jmp quickos_kernel_entry

section .bss
resb 4096
stack:

mb2_info_ptr:
resb 4
