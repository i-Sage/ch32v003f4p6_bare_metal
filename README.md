# Boot up

When our MCU first boots up, when power arrives at the core, the MCU starts executing code from a specific memory address. We need to tell the linker where to put our code so that the chip can find it. We also need to set up the stack pointer, initialize any variables that need specific starting values, and then jump into our Rust code. This happens before the Rust run time knows it even exists.

## Linker Scripts

A linker script controls every link. It's written in the linker command language. The main purpose of the linker script is to describe how the sections in the input file should be mapped into the output file, and to control the memory layout of the output file. However, when necessary, the linker script can also direct the linker to perform many other operations, using the linker commands.

### Important Linker Script Concepts

The linker combines input files into a single output file. The output file and each input file are in a specific data format known as an object format. Each file is called an object file. The output file is often called an executable, but for our case, we call it an object file. Each object file has, among other things, a list of sections. We sometimes refer to a section in an input file as an input section; similarly, a section in the output file is an output section. Each section in an object file has a name and a size. Most sections also have an associated block of data, known as the section contents. A section may be marked loadable, meaning that the contents should be loaded into memory when the output file is run. A section with no contents may be allocatable, which means that an area in memory should be set aside, but nothing in particular should be loaded there (in some cases this memory must be zeroed out). A section which is neither loadable or allocatable, typically contains some sort of debugging information.

Every loadable or allocatable output section has two addresses. The first is the VMA, or virtual memory address. This is the address at which the section will be loaded. In most cases the two addresses will be the same. An example of when they might be different is when a data section is loaded into ROM, and then copied into RAM when the progra starts up (this technique is often used to initialise global variables in a ROM based system). In this case the ROM address would be the LMA, and the RAM address would be the VAM.

We can see the sections in an object file by using the `objdump` program with the `-h` option.

Every object file also has a list of symbols, known as the symbol table. A symbol may be defined or undefined. Each symbol has a name, and each defined symbol has an address, among other information. If we compile a C or C++ program into an object file, we will get a definition symbol for every defined function and global or static variable. Every undefined function or global varialbe, which is referenced in the input file, will become an undefined symbol. We can see the symbols in an object file by using the `nm` program, or by using the `objdump` program with the `-t` option.

### Simple Linker Script Commands

#### The Entry Point

The first instruction to execute in a program is called the entry point. You can use the `ENTRY` linker script command to set the entry point. The argument is a symbol name:

`ENTRY ( symbol )`

There are several ways to set the entry point. The linker will set the entry point by trying each of the following methods in order, and stopping when one of them succeeds:

- The `-e` entry command-line option;
- The `ENTRY (symbol)` command in a linker script;
- The value of the symbol, `start` if defined;
- The address of the first byte of the `.text` section, if present;
- The address, `0`.

#### *SECTIONS* COMMAND

The `SECTIONS` command tells the liker how to map input sections into output sections, and how to place the output sections in memory. The format of the `SECTIONS` command is:

```linker
SECTIONS
{
  sections - command
  sections - command
}
```

Each sections-command may be one of the following:

- An `ENTRY` command
- A Symbol assignment
- An output section description
- An overlay description

[Learn more about linker scripts](https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html)

### Memory-Mapped Register Addresses

*the heart of embedded programming: memory-mapped I/O*

In our MCU the ch32v003f4p6 in our case, peripherals (like GPIO, UART, timers, e.t.c) don't have special instructions to control them. Instead, they're controlled by reading and writing to specific memory addresses. The hardware is designed so that when we write to these special registers, we're not writing to RAM-we're we're writing to the GPIO peripheral's registers.

We can think of it like this: The address space of the processor is divded into regions:

- 0x00000000 - 0x00003FFF: Flash memory (16KB)
- 0x20000000 - 0x200007FF: RAM (2KB)
- 0x40000000 - 0x4FFFFFFF: Peripherals (memory mapped registers)

!NOTE: incomplete for now

### The Blink Program

#### The Clock System

When our MCU 

```bash
cargo build --release
rust-objcopy -O binary target/riscv32imac-unknown-none-elf/release/ch32v003f4p6_bare_metal firmware.bin
~/.platformio/packages/tool-openocd-riscv-wch/bin/openocd \
  -f ~/.platformio/packages/tool-openocd-riscv-wch/bin/wch-riscv.cfg \
  -c init \
  -c halt \
  -c "program firmware.bin 0x00000000 verify" \
  -c "reset" \
  -c "exit"
```
