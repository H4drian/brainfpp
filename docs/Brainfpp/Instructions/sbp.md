# Sbp
- Description: Subtracts n from data pointer
- Keyword: `sbp`
- Lexem Token: `Token::Sbp`
- Compiles to: `<(n many)`
- Takes Argument (n): true
- Argument Type: 16-bit unsigned integer

## Example
```
sdp 10 # set data pointer to 10
sbp 10 # decrement pointer by 10 (back to cell 0)
ldc 48
```