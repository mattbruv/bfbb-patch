# Thoughts

- If you append the strings & relocations at the end, you shouldn't need to update all of the existing data.
  - Only will need to remove relocations that are no longer relevant, and then append new ones at the end.
  - new relocation strings are appended to end of string table

# Sections in our Assembly Files

## Repated Sections

These are the only sections that are repeated (appear more than once with the same exact section name) in our object files:

- .rela.text
- .sdata
- .text

The objpatch tool will need to consolidate these

## List of all sections

Here is an exhaustive list of all the section types that are found in all of our `.s` files on encounter's the `redisasm` branch.
It also includes my research to understand what each one does

- .bss

  - contains statically allocated variables that are declared but have not been assigned a value yet.

- .ctors

  - [.init, .ctors, and .init_array](https://maskray.me/blog/2021-11-07-init-ctors-init-array)
  - This section holds an array of the [global constructor function pointers](http://ftp.math.utah.edu/u/ma/hohn/linux/misc/elf/node4.html) of a program.
  - ctors is found in 9 files:
    - xpkrsvc.o
    - xScrFx.o
    - zEntCruiseBubble.o
    - zEntPlayer.o
    - zNPCGlyph.o
    - zNPCHazard.o
    - zTalkBox.o
    - zUI.o
    - \_\_init_cpp_exceptions.o

- .data
  - where global tables, variables, etc. live.
- .dtors

  - [.init, .ctors, and .init_array](https://maskray.me/blog/2021-11-07-init-ctors-init-array)
  - This section holds an array of the global destructor function pointers of a program.
  - dtors is found in 2 files:
    - global_destructor_chain.o
    - \_\_init_cpp_exceptions.o

- .init
  - [.init, .ctors, and .init_array](https://maskray.me/blog/2021-11-07-init-ctors-init-array)
  - init is found in 6 files:
    - dolphin_trk.o
    - mem_TRK.o
    - \_\_exception.o
    - \_\_mem.o
    - \_\_ppc_eabi_init.o
    - \_\_start.o
- .rela.ctors
- .rela.data
- .rela.dtors
- .rela.init
- .rela.rodata
- .rela.sdata
- .rela.sdata2
- .rela.text
- .relaextab
- .relaextabindex
- .rodata
  - that's where your strings go
- .sbss
  - This section holds uninitialized data that contribute to the program's memory image. Data objects contained in this section are recommended to be eight bytes or less in size. The system initializes the data with zeroes when the program begins to run.
- .sbss2
- .sdata
  - This section holds initialized data that contribute to the program's memory image.
- .sdata2
- .shstrtab
- .strtab
  - String table sections hold null-terminated character sequences, commonly called strings. The object file uses these strings to represent symbol and section names. One references a string as an index into the string table section.
- .symtab

  - An object file's symbol table holds information needed to locate and relocate a program's symbolic definitions and references. A symbol table index is a subscript into this array.

- .text

  - Executable program code
  - Can have multiple .text sections per file

- extab

  - probably not relevant to patch
  - only found in Gecko_ExceptionPPC.o, New.o, NMWException.o

- extabindex
