# Building Rift Core

## B0 toolchain

The B0 build requires:

- Rust and Cargo (stable toolchain)
- The Rust `x86_64-unknown-none` target: `rustup target add x86_64-unknown-none`
- GNU Make

From the repository root, run `make` to build the release kernel ELF. Other useful targets are `make check`, `make fmt`, and `make clean`. Set `TARGET` to override the Rust target when appropriate.

The ELF is emitted at `target/x86_64-unknown-none/release/rift-kernel`. `kernel/linker.ld` places the image at 1 MiB and defines the kernel section boundaries. The current `_start` is a link-test stub only: it does not initialize the CPU or satisfy a boot protocol, so the ELF cannot yet be launched by firmware or an emulator.

## External tools and next milestone

QEMU and ISO/image tooling are not needed for B0. Before B1 boot testing, the boot protocol will be selected and documented; that milestone will determine whether a bootloader dependency and QEMU are needed. Do not treat the current ELF as a runnable image.
