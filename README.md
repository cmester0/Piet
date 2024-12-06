# Piet
Working Piet (https://www.dangermouse.net/esoteric/piet.html) interpreter and stk to Piet compiler

# Installation
Uses pypng for reading and writing images.

# Run
Call
```bash
python cli <path_to_file.smpl> ([gen||stk||piet||debug],)*
```

# TODO
[] Add IEEE float represented by ints on the stack with the same value

# SMPL (library) Instructions
Assuming correct heap / stack layout. So pushing is not the top of the stack, but under thee stack size.

## Malloc
A number of elements to alocate (memory will be allocated at memory location 0)

## Set Heap
Push a number, and then the location onto the stack. Will put the number onto that heap location.

## Get Heap
Push the location onto the stack. Will put the number onto that heap location.

## Copy memory
Takes two arguments on the stack, runs until they are equal

