# Rift Core
A custom operating system written in Rust, C, and Assembly.

## Architecture
- **Kernel:** Hybrid Rust/C/ASM. Rust for safe logic/memory, C for hardware glue, ASM for boot/interrupts.
- **Boot:** Custom x86_64 bootloader (or Limine integration).

## Building
Requires: `nasm`, `gcc`, `rustc`, `cargo`, `make`, `xorriso` (for ISO creation), and a working `limine-deploy` tool for the final image.

To build the kernel binary:

```bash
make kernel
```

To stage a Limine boot image:

```bash
make iso
```

The staging output lives under `build/iso/boot` and is ready for `limine-deploy` to create a bootable ISO.
