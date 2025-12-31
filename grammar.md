# SimplC Grammar

```
<program>    ::= <function>
<function>   ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
<statement>  ::= "return" <exp> ";"
<expr>       ::= <int> | <unop> <expr> | "(" <expr> ")"
<unop>       ::= "-" | "~"
<identifier> ::= ? An identifier token ?
<int>        ::= ? A constant token ?
```

