 .section .text.boot,"ax",@progbits
.balign 16

.global _start
.type _start, @function
_start:
    /* Limine enters in 64-bit mode with a valid stack. Switch to our own. */
    cli
    cld
    leaq boot_stack_top(%rip), %rsp
    andq $-16, %rsp
    xorq %rbp, %rbp

    call rust_kernel_main

    /* The Rust entry point diverges; never return to the bootloader. */
    ud2
.size _start, . - _start

.section .bss.stack,"aw",@nobits
.balign 16
.global boot_stack_bottom
boot_stack_bottom:
    .skip 65536
.global boot_stack_top
boot_stack_top:
