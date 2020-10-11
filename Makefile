CC := clang

QEMU := qemu-system-x86_64
QEMU_ARGS := \
	-bios OVMF.fd \
	-hda fat:rw:mnt

MNT := mnt
SRC := src
TMP := tmp
BOOT_DIR := ${MNT}/EFI/BOOT
BOOTX64  := ${BOOT_DIR}/BOOTX64.EFI

EFI_SOURCES = ${SRC}/efi_main.c
EFI_MAIN_O = ${TMP}/efi_main.o

SOURCES := ${SRC}/*.c

CLANG_FLAGS= \
	-target x86_64-pc-win32-coff \
	-fno-stack-protector -fshort-wchar \
	-mno-red-zone \
	-nostdlibinc \
	-nostdlib \
	-Wall -Wpedantic \
	-Wl,-entry:efi_main \
	-Wl,-subsystem:efi_application \
	-fuse-ld=lld-link \
	-o ${BOOTX64} \
	${EFI_SOURCES}

# --


run: build
	${QEMU} ${QEMU_ARGS}

build:
	mkdir -p ${BOOT_DIR} ${TMP}
	${CC} ${CLANG_FLAGS}

lean:
	rm -rf ${MNT}
	rm -rf ${TMP}

help:
	@echo Fenix
	@echo "  run   : Run fenix on qemu"
	@echo "  build : build FENIX.EFI"
	@echo "  clean : Clean all"
