# Rift Core

Rift Core is an x86-64 operating-system project with Rust as its primary kernel language. Implementation follows the staged B0–B7 and V1–V7 sequence in the project architecture plan; the architecture tree is a target, not a requirement to create every subsystem in advance.

## Current milestone: B0 — build system and linker

B0 establishes a Cargo workspace, an x86-64 freestanding kernel target, and the initial ELF linker layout. The kernel entry is only a link-test stub and is **not bootable yet**. Boot protocol selection and CPU entry setup are B1 work.

## Build

Install Rust and the freestanding target, then run:

```sh
rustup target add x86_64-unknown-none
make
```

The linked kernel ELF is written to `target/x86_64-unknown-none/release/rift-kernel`. See [the build notes](docs/development/build.md) for available targets and current limitations.
