# SimplC Grammar

```
<program>     ::= <function>
<function>    ::= "int" <identifier> "(" "void" ")" "{" { <block> } "}"
<block>       ::= <statement> | <declaration>
<statement>   ::= "return" <exp> ";" | <expr> ";" | ";" 
                | "if" "(" <expr> ")" <statement> [ "else" <statement> ]
<declaration> ::= "int" <identifier> [ "=" <expr> ] ";"
<expr>        ::= <factor> | <expr> <binop> <expr> 
<factor>      ::= <int> | <identifier> | <unop> <factor> | "(" <expr> ")" 
<unop>        ::= "-" | "~" | "!" | <increment>
<increment>   ::= "++" | "--"
<identifier>  ::= ? An identifier token ?
<int>         ::= ? A constant token ?
<binop>       ::= "+" | "-" | "*" | "/" | "%" | "&" | "|" | "^" | "<<" | ">>" | "&&" | "||" | "==" | "!=" |
                 "<" | "<=" | ">" | ">=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>"
```

