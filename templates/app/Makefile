# Makefile — Link Rust staticlib + C glue into an AmigaOS 4 application
# Linked with clib4 and -lauto (auto-open libraries).
# Run inside Docker: walkero/amigagccondocker:os4-gcc11

TARGET    = myapp
CC        = ppc-amigaos-gcc

# SDK include paths
SDK_INC   = /opt/ppc-amigaos/ppc-amigaos/SDK/include/include_h

# C glue from amigaos4-sys crate (varargs wrappers)
SYS_GLUE_SRC = ../../amigaos4-sys/glue/amiga_glue.c
SYS_GLUE_OBJ = build/sys_glue.o

# Rust output (staticlib built on host)
RUST_LIB  = target/powerpc-amigaos/release/libmyapp.a

# Compiler flags for C glue
CFLAGS    = -mcrt=clib4 -O2 -Wall -I$(SDK_INC)

# Linker flags
LDFLAGS   = -mcrt=clib4
LIBS      = -lpthread -lauto

.PHONY: all clean

all: build $(TARGET)

build:
	mkdir -p build

$(SYS_GLUE_OBJ): $(SYS_GLUE_SRC)
	$(CC) $(CFLAGS) -c $< -o $@

$(TARGET): $(SYS_GLUE_OBJ) $(RUST_LIB)
	$(CC) $(LDFLAGS) -o $@ $(SYS_GLUE_OBJ) $(RUST_LIB) $(LIBS)

clean:
	rm -rf build $(TARGET)
