# lex_str
- function: `bfpp::lexer::lex_str``
- parameter(s): `source_code: &str, linked_libs: Vec<&str>`
- returns: `Vec<Lexem>`

The `lex_str` function is used to take brainfpp source code and break it down into its
lexical tokens (Lexems). `lex_str` returns `Vec<Lexem>` as opposed to `lex_str_to_string`
which formats the output as a `String` instead of a vector of lexems.