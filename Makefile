ARCH := x86_64
TARGET ?= x86_64-unknown-none
CARGO ?= cargo
KERNEL_ELF := target/$(TARGET)/release/rift-kernel

.PHONY: all kernel check fmt clean

all: kernel

kernel:
	$(CARGO) build --workspace --target $(TARGET) --release
	@echo "Kernel ELF built: $(KERNEL_ELF)"

check:
	$(CARGO) check --workspace --target $(TARGET)

fmt:
	$(CARGO) fmt --all -- --check

clean:
	$(CARGO) clean
	rm -rf build
