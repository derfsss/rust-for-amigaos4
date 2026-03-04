# Makefile — Link Rust staticlib + C glue into an AmigaOS handler/driver
# No CRT, no clib4, no -lauto. Driver mode: -nostartfiles -nodefaultlibs -lgcc
# Run inside Docker: walkero/amigagccondocker:os4-gcc11

TARGET    = hello-driver
CC        = ppc-amigaos-gcc

# SDK include paths
SDK_INC   = /opt/ppc-amigaos/ppc-amigaos/SDK/include/include_h

# C glue sources
DRIVER_GLUE_SRC = src/driver_glue.c
SYS_GLUE_SRC    = ../../amigaos4-sys/glue/amiga_glue.c

# C glue objects
DRIVER_GLUE_OBJ = build/driver_glue.o
SYS_GLUE_OBJ    = build/sys_glue.o

# Rust output (staticlib built on host)
RUST_LIB  = target/powerpc-amigaos/release/libhello_driver.a

# Compiler flags — NO CRT, driver-compatible
# -ffunction-sections/-fdata-sections + --gc-sections strips unused glue
CFLAGS    = -O2 -Wall -D__AMIGAOS4__ -U__USE_INLINE__ -I$(SDK_INC) \
            -fno-tree-loop-distribute-patterns -ffunction-sections -fdata-sections

# Linker flags — driver mode (no CRT, no default libs)
LDFLAGS   = -nostartfiles -nodefaultlibs -Wl,--gc-sections
LIBS      = -lgcc

.PHONY: all clean

all: build $(TARGET)

build:
	mkdir -p build

$(DRIVER_GLUE_OBJ): $(DRIVER_GLUE_SRC)
	$(CC) $(CFLAGS) -c $< -o $@

$(SYS_GLUE_OBJ): $(SYS_GLUE_SRC)
	$(CC) $(CFLAGS) -c $< -o $@

$(TARGET): $(DRIVER_GLUE_OBJ) $(SYS_GLUE_OBJ) $(RUST_LIB)
	$(CC) $(LDFLAGS) -o $@ $(DRIVER_GLUE_OBJ) $(SYS_GLUE_OBJ) $(RUST_LIB) $(LIBS)

clean:
	rm -rf build $(TARGET)
