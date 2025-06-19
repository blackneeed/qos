ENTRY(kloader_main)

SECTIONS 
{
    .multiboot ALIGN(4K) : {
        *(.multiboot*)
    }

    . = 1M;
    PROVIDE(KERNEL_START = .);

    .text ALIGN(4K) : {
        *(.text*)
    }

    .rodata ALIGN(4K) : {
        *(.rodata*)
    }

    .data ALIGN(4K) : {
        *(.data*)
    }

    .bss ALIGN(4K) : {
        *(COMMON*)
        *(.bss*)
    }

    PROVIDE(KERNEL_END = .);
}
