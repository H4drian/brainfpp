# compile_str_optimized
- function: `bfpp::compiler::compile_str_optimized`
- parameter(s): `source_code: &str, linked_libs: Vec<&str>`
- returns: `String`

The `compile_str_unoptimized` function will take brainfpp source code as &str and return it as
brainfuck code. The function also takes in a list of filepaths (`linked_libs: Vec<&str>`)
which are included with compilation. The function does optimize the compiled code.