# SimplC Grammar

```
<program>    ::= <function>
<function>   ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
<statement>  ::= "return" <exp> ";"
<expr>       ::= <factor> | <expr> <binop> <expr>
<factor>     ::= <int> | <unop> <factor> | "(" <expr> ")" 
<unop>       ::= "-" | "~"
<identifier> ::= ? An identifier token ?
<int>        ::= ? A constant token ?
<binop>      ::= "+" | "-" | "*" | "/" | "%"
```

