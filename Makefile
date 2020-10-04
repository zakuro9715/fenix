LD := "/mnt/c/Program Files (x86)/Microsoft Visual Studio/2019/Community/VC/Tools/Llvm/x64/bin/lld-link.exe"
CC := clang

LINKER=lld
QEMU := qemu-system-x86_64
EFI := FENIX.EFI

QEMU_ARGS= \
	-bios OVMF.fd \
	-hda fat:rw:mnt \

BOOT_DIR=${MNT}/EFI/BOOT
MNT := mnt
SRC := src
TMP := tmp

EFI_MAIN_C = ${SRC}/efi_main.c
EFI_MAIN_O = ${TMP}/efi_main.o

SOURCES := ${SRC}/*.c

CLANG_FLAGS= \
	-target x86_64-pc-win32-coff \
	-fno-stack-protector -fshort-wchar \
	-mno-red-zone \
	-nostdlibinc \
	-Wall -Wpedantic \
	-c -o ${EFI_MAIN_O} ${EFI_MAIN_C}

LINKER_FLAGS := \
	-subsystem:efi_application -nodefaultlib \
	-entry:efi_main ${EFI_MAIN_O} -out:${BOOT_DIR}/BOOTX64.EFI


# --


run: build
	${QEMU} ${QEMU_ARGS}

build:
	mkdir -p ${BOOT_DIR} ${TMP}
	${CC} ${CLANG_FLAGS}
	${LD} ${LINKER_FLAGS}

clean:
	rm -rf ${MNT}
	rm -rf ${TMP}

help:
	@echo Fenix
	@echo "  run   : Run fenix on qemu"
	@echo "  build : build FENIX.EFI"
	@echo "  clean : Clean all"
