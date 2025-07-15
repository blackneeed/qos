[bits 32]
[global kloader_main]

gdt_start:
    dq 0
    dq 0x00CF9A000000FFFF
    dq 0x00CF92000000FFFF
gdt_descriptor:
    dw $ - gdt_start - 1
    dd gdt_start

section .multiboot
align 8
multiboot2_start:
dd 0xE85250D6
dd 0
dd multiboot2_end - multiboot2_start
dd -(0xE85250D6 + 0 + (multiboot2_end - multiboot2_start))

align 8
multiboot2_mmap_tag_start:
dw 6
dw 0
dd multiboot2_mmap_tag_end - multiboot2_mmap_tag_start
multiboot2_mmap_tag_end:

align 8
multiboot2_fb_tag_start:
dw 5
dw 1
dd multiboot2_fb_tag_end - multiboot2_fb_tag_start
dd 0
dd 0 
dd 32
multiboot2_fb_tag_end:

align 8
multiboot2_acpi_new_tag_start:
dw 15
dw 1
dd multiboot2_acpi_new_tag_end - multiboot2_acpi_new_tag_start
multiboot2_acpi_new_tag_end:

align 8
multiboot2_acpi_old_tag_start:
dw 14
dw 1
dd multiboot2_acpi_old_tag_end - multiboot2_acpi_old_tag_start
multiboot2_acpi_old_tag_end:

align 8
multiboot2_end_tag_start:
dw 0
dw 0
dd multiboot2_end_tag_end - multiboot2_end_tag_start
multiboot2_end_tag_end:
multiboot2_end:

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
resb 65536
stack:

mb2_info_ptr:
resb 4
