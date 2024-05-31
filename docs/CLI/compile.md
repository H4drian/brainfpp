# Compile

```
brainfpp.exe-compile 
compiles a brainfpp program to brainf

USAGE:
    brainfpp.exe compile [OPTIONS] <INFILE>

ARGS:
    <INFILE>    input file to compile

OPTIONS:
    -d, --dev                  builds the program in dev mode (unoptimized)
    -h, --help                 Print help information
    -l, --link <lib>...        links a brainfpp file to the compiler
        --no-std               dissable default linking of brainfpp standard library
    -o, --outfile <OUTFILE>    the output file to write to
    -r, --release              builds the program in release mode (optimized)
```

The compile subcommand takes a brainfpp program and compiles it to a normal brainfuck 
program. 
The subcommand will either output the compiled program to a file if the -o/--outfile option is used or will default to printing the output to the command line.
If the brainfpp program being compiled is dependent on other files its importent to link all of them using the -l --link option. Brainfpp programs can be linked or `synapse.toml` files can be linked.