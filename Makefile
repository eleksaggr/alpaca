ARCH ?= x86_64
KERNEL := build/kernel-$(ARCH).bin
ISO := build/os.iso
TARGET := $(ARCH)-alpaca

LINKER := arch/x86_64/linker.ld
SOURCES := $(wildcard arch/$(ARCH)/*.asm)
OBJECTS := $(patsubst arch/$(ARCH)/%.asm, build/arch/$(ARCH)/%.o, $(SOURCES))
LIB := target/$(TARGET)/debug/libalpaca.a

.PHONY: all clean kernel run

all: $(KERNEL)

kernel:
	@RUST_TARGET_PATH=$(shell pwd) xargo build --target=$(TARGET)

clean:
	@rm -rf build

# Preferably we would boot the kernel with QEMU's "-kernel" option, but it seems Multiboot2 is not supported.
$(ISO): $(KERNEL) $(LIB)
	@mkdir -p build/iso/boot/grub
	@echo -e "set timeout=0\nset default=0\n\nmenuentry \"Alpaca\" {\n\tmultiboot2 /boot/kernel.bin\n\tboot\n}" > build/iso/boot/grub/grub.cfg
	@cp $(KERNEL) build/iso/boot/kernel.bin
	@grub-mkrescue build/iso -o $(ISO)

run: $(ISO)
	@qemu-system-x86_64 -cdrom $(ISO) -serial file:debug.log

$(KERNEL): kernel $(OBJECTS) $(LINKER) $(LIB)
	@ld -n -T $(LINKER) -o $(KERNEL) $(OBJECTS) $(LIB)

build/arch/$(ARCH)/%.o: arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
