# Lexem
- struct: `bfpp::lexer::lexem::Lexem`
- fields:
-   - token (`bfpp::lexer::lexem::Token`) 
    - arg (`Option<usize>`)
    - line (`usize`)

The `Lexem` struct is used by the lexer and compiler as the middle-end for brainfpp. It 
breaks down every line of source code into an instruction token, argument option, and 
line. 

## Methods
- Lexem::new()