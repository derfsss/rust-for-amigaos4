# Makefile — Link Rust staticlib + C glue into an AmigaOS 4 exec device
# No CRT, no clib4. Device mode: -nostartfiles -nodefaultlibs -lgcc
# Run inside Docker: walkero/amigagccondocker:os4-gcc11

TARGET    = ram.device
CC        = ppc-amigaos-gcc

# SDK include paths
SDK_INC   = /opt/ppc-amigaos/ppc-amigaos/SDK/include/include_h

# C glue sources
DEV_GLUE_SRC    = src/device_glue.c
SYS_GLUE_SRC    = ../../amigaos4-sys/glue/amiga_glue.c

# C glue objects
DEV_GLUE_OBJ    = build/device_glue.o
SYS_GLUE_OBJ    = build/sys_glue.o

# Rust output (staticlib built on host)
RUST_LIB  = target/powerpc-amigaos/release/libram_device.a

# Compiler flags
CFLAGS    = -O2 -Wall -D__AMIGAOS4__ -U__USE_INLINE__ -I$(SDK_INC) \
            -fno-tree-loop-distribute-patterns -ffunction-sections -fdata-sections

# Linker flags — device mode (same as library mode)
# --undefined=RomTag forces the Resident struct to be retained as a GC root
# (otherwise --gc-sections drops it — nothing statically references the RomTag,
# AmigaOS finds it by scanning memory for RTC_MATCHWORD).
LDFLAGS   = -nostartfiles -nodefaultlibs -Wl,--gc-sections -Wl,--undefined=RomTag
LIBS      = -lgcc

.PHONY: all clean

all: build $(TARGET)

build:
	mkdir -p build

$(DEV_GLUE_OBJ): $(DEV_GLUE_SRC)
	$(CC) $(CFLAGS) -c $< -o $@

$(SYS_GLUE_OBJ): $(SYS_GLUE_SRC)
	$(CC) $(CFLAGS) -c $< -o $@

$(TARGET): $(DEV_GLUE_OBJ) $(SYS_GLUE_OBJ) $(RUST_LIB)
	$(CC) $(LDFLAGS) -o $@ $(DEV_GLUE_OBJ) $(SYS_GLUE_OBJ) $(RUST_LIB) $(LIBS)
	ppc-amigaos-strip $@

clean:
	rm -rf build $(TARGET)
