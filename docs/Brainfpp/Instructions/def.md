# Def/Edef
- Description: Defines a subroutine which can be called later
- Keyword(s): `def` and `edef`
- Lexem Token(s): `Token::Def` and `Token::Edef`
- Compiles to: N/A (Used in lexer phase)
- Takes Argument (n): true
- Argument Type: brainfpp code

## Example
```
def my_func
    ldc 10
    out
edef
```