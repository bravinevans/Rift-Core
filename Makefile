ARCH ?= x86_64
BUILD_DIR := build
KERNEL_MANIFEST := kernel/Cargo.toml
TARGET := $(ARCH)-unknown-none
KERNEL_ELF := kernel/target/$(TARGET)/release/rift-kernel
ISO_DIR := $(BUILD_DIR)/iso

.PHONY: all kernel iso clean

all: iso

kernel:
	cargo build --manifest-path $(KERNEL_MANIFEST) --target $(TARGET) --release

iso: kernel
	mkdir -p $(ISO_DIR)/boot
	cp $(KERNEL_ELF) $(ISO_DIR)/boot/rift-kernel.elf
	cp boot/limine.cfg $(ISO_DIR)/boot/limine.cfg
	@echo "Limine ISO staging complete. Use limine-deploy $(ISO_DIR) to produce the final disk image."

clean:
	cargo clean --manifest-path $(KERNEL_MANIFEST)
	rm -rf $(BUILD_DIR)
