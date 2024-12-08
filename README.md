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
Takes 4 arguments,
- the heap index of the new list,
- the heap index of the old list,
- end offset
- start offset

# Stack Layout
If we look at some stack for piet
```piet
[1, 1, 114, 2, 0, 0, 0, 7, -1, 110, 115, 113, 3, 119, 2, 3, 0, 1, 0, 12]
```
this is given as
## Heap:
- elements:  [1, 1, 114, 2, 0, 0, 0]
  + list: [1, 1, 114]
    * allocated: 1
    * filled in: 1
	* Elements: 114
  + list: [2, 0, 0, 0]
    * allocated: 2
    * filled in: 0
	* Elements: [0 0]
- of size:   7
## Stack:
- variables: -1
- elements:  [110, 115, 113, 3, 119, 2, 3, 0, 1, 0]
- size       12
