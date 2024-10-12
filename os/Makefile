# Building
TARGET := riscv64gc-unknown-none-elf
MODE := debug
KERNEL_ELF := target/$(TARGET)/$(MODE)/os

# BOARD
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

ifeq ($(MODE), release) 
	MODE_ARG := --release
endif
kernel:
	cargo build --$(MODE_ARG)

clean:
	cargo clean

run: kernel
	timeout --foreground 30s qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-kernel $(KERNEL_ELF)

gdb: 
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-kernel $(KERNEL_ELF) \
		-s -S &
	@gdb-multiarch \
		-ex "file $(KERNEL_ELF)" \
		-x gdb.init

.PHONY: build kernel clean run
