# Comments
- Description: Comments ignored by compiler.
- Keyword: `#` and `#- -#`
- Lexem Token: N/A (ignored)
- Compiles to: N/A (ignored)
- Takes Argument: N/A (ignored)
- Argument Type: N/A (ignored)

Comments in brainfpp are used to label parts of a program without interfearing with 
functional code. Comments in brainfpp can be either single line (`#`) or multi-line
(`#- -#`). Single line comments exist on a single line of code. Example: 
```
# this program prints hi
ldc 104
out
add 1
out
```
While multi-line comments are able to exist on multiple lines. Example:
```
#-
    This program prints hi. 
    It does this by loading the value for h into cell 0
    then outputing it. Then adding 1 to cell 1 to get the
    value for i then outputs it.
-#
ldc 104
out
add 1
out
```

Along with explicite comments there are also implicite comments. If some text does not fall into the syntax format or uses any of the instruction keywords it is treated as a comment. This is not recommended.
```
this acts as an implicite comment. This is not recommended but is still possible
ldc 104
out
add 1
out
```