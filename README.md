# Brainfpp: Brainfuck Improved

An assembly language that compiles to Brainfuck Turing machine code.

## Syntax

Code is very simple. Each instruction is separated by a new line and follows an `<instruction><arg>` format. There are 7 instructions:

- `sdp <addr>`: Set data pointer
- `adp <val>`: Add to data pointer
- `sbp <val>`: Subtract from pointer
- `ldc <val>`: Load value into current cell
- `add <val>`: Add value
- `sub <val>`: Subtract value
- `out`: Output current value
- `inn`: Input value
- `[]`: Loop
- `def/edef`: Define subroutine
- `call`: Call subroutine
- `#`: Starts a single line comment. This can be at the start of a line or end but
never between an instruction and arg.
- `#- -#`: Starts and ends a multi line comment. Everything between these will be
ignored.

An example of a brainfpp program that says hi\n would go as follows

```
def set_cell_zero
    [
        sub 1
    ]
edef

sdp 0               set data pointer to 0
add 104             set cell 0 to 104 (h in ascii)
out                 output cell 0
add 1               add 1 to cell 0 (i in ascii)
out                 output cell 0
call set_cell_zero
add 10              add 10 to cell 0 (newline char)
out                 output 
```