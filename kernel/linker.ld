OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)

/* Limine non-relocatable executables must use higher-half virtual addresses. */
KERNEL_VMA = 0xffffffff80000000;

PHDRS
{
    text   PT_LOAD FLAGS(5); /* Read and execute. */
    rodata PT_LOAD FLAGS(4); /* Read only. */
    data   PT_LOAD FLAGS(6); /* Read and write. */
}

SECTIONS
{
    . = KERNEL_VMA;
    __kernel_start = .;

    .text ALIGN(4K) :
    {
        KEEP(*(.text.boot))
        KEEP(*(.text._start))
        *(.text .text.*)
    } :text

    .rodata ALIGN(4K) :
    {
        *(.rodata .rodata.*)
    } :rodata

    /* Limine updates the request response pointers before entering the kernel. */
    .limine_requests ALIGN(8) :
    {
        KEEP(*(.limine_requests_start))
        KEEP(*(.limine_requests))
        KEEP(*(.limine_requests_end))
    } :data

    .data ALIGN(4K) :
    {
        *(.data .data.*)
        *(.got .got.*)
    } :data

    .bss ALIGN(4K) (NOLOAD) :
    {
        __bss_start = .;
        *(COMMON)
        *(.bss .bss.*)
        . = ALIGN(4K);
        __bss_end = .;
    } :data

    __kernel_end = .;

    /DISCARD/ :
    {
        *(.eh_frame .eh_frame.*)
        *(.gcc_except_table .gcc_except_table.*)
        *(.comment)
        *(.note .note.*)
    }
}
