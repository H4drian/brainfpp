# Subroutine
- struct: `bfpp::lexer::lexem::Subroutine`
- fields:
-   - name (`String`) 
    - code (`Vec<bfpp::lexer::lexem::Lexem>`)

The `Subroutine` struct is used by the lexer to keep track of defined subroutines and the
code that goes with them. It holds the name of the subroutine and the code for it.

## Methods
- Subroutine::new()