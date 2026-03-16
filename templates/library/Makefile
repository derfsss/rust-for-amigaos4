# Makefile — Link Rust staticlib + C glue into an AmigaOS 4 shared library
# No CRT, no clib4. Library mode: -nostartfiles -nodefaultlibs -lgcc
# Run inside Docker: walkero/amigagccondocker:os4-gcc11

TARGET    = PROJNAME.library
CC        = ppc-amigaos-gcc

# SDK include paths
SDK_INC   = /opt/ppc-amigaos/ppc-amigaos/SDK/include/include_h

# C glue sources
LIB_GLUE_SRC    = src/library_glue.c
SYS_GLUE_SRC    = ../../amigaos4-sys/glue/amiga_glue.c

# C glue objects
LIB_GLUE_OBJ    = build/library_glue.o
SYS_GLUE_OBJ    = build/sys_glue.o

# Rust output (staticlib built on host)
RUST_LIB  = target/powerpc-amigaos/release/libPROJNAME.a

# Compiler flags — NO CRT, library-compatible
CFLAGS    = -O2 -Wall -D__AMIGAOS4__ -U__USE_INLINE__ -I$(SDK_INC) \
            -fno-tree-loop-distribute-patterns -ffunction-sections -fdata-sections

# Linker flags — library mode (no CRT, no default libs)
LDFLAGS   = -nostartfiles -nodefaultlibs -Wl,--gc-sections
LIBS      = -lgcc

.PHONY: all clean

all: build $(TARGET)

build:
	mkdir -p build

$(LIB_GLUE_OBJ): $(LIB_GLUE_SRC)
	$(CC) $(CFLAGS) -c $< -o $@

$(SYS_GLUE_OBJ): $(SYS_GLUE_SRC)
	$(CC) $(CFLAGS) -c $< -o $@

$(TARGET): $(LIB_GLUE_OBJ) $(SYS_GLUE_OBJ) $(RUST_LIB)
	$(CC) $(LDFLAGS) -o $@ $(LIB_GLUE_OBJ) $(SYS_GLUE_OBJ) $(RUST_LIB) $(LIBS)
	ppc-amigaos-strip $@

clean:
	rm -rf build $(TARGET)
