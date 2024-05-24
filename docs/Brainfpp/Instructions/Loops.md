# []
- Description: Creates a JNZ (jump if not zero) loop
- Keyword(s): `[` and `]`
- Lexem Token: `Token::Slp` and `Token::Elp``
- Compiles to: `[` and `]`
- Takes Argument (n): false
- Argument Type: N/A (no arg)

## Example
```
[
    # subtract 1 from current cell until equal to zero
    sub 1
]
out # output ASCII NULL
```