# Syntax

Brainfpp syntax is based on assembly language syntax. Any functional 
line of code in brainfpp is formated as:
```
<Instruction> <Arg?>
```
Depending on the instruction an argument might not be needed. If an argument is fed to it it will most likely act as a comment.

Indentation is not accounted for by the compiler. So a program like:
```
[
    sub 1
]
```
acts the same as:
```
[
sub 1
]
```

Instructions are not case sensitive. So ADD acts the same as add or AdD, etc. However subroutines/functions ARE case senitive. So a function defined as my_func cannot be called as MY_FUNC.