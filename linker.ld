ENTRY(quickos_loader_entry)

SECTIONS 
{
    
    . = 1M;
    KERNEL_START = .;

    .text BLOCK(4K) : ALIGN(4K) {
        *(.multiboot)
        *(.text)
    }

    .data BLOCK(4K) : ALIGN(4K) {
        *(.data)
    }

    .rodata BLOCK(4K) : ALIGN(4K) {
        *(.rodata)
    }

    .bss BLOCK(4K) : ALIGN(4K) {
        *(COMMON)
        *(.bss)
        *(.bootstrap_stack)
    }

    .note-gnu-build-id BLOCK(4K) : ALIGN(4K) {
        *(.note-gnu-build-id)
    }
    KERNEL_END = .;
}
