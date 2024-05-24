# Lex

```
brainfpp.exe-lex 
returns the lexems of a brainfpp program

USAGE:
    brainfpp.exe lex [OPTIONS] <INFILE>

ARGS:
    <INFILE>    input file to compile

OPTIONS:
    -h, --help                 Print help information
    -l, --link <lib>...        links a brainfpp file to the compiler
        --no-std               dissable default linking of brainfpp standard library
    -o, --outfile <OUTFILE>    the output file to write to
```

The lex subcommand takes a brainfpp program file and outputs the middle-end lexed version
of it. 