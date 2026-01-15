# SimplC Grammar

```
<program>     ::= <function>
<function>    ::= "int" <identifier> "(" "void" ")" "{" { <block> } "}"
<block>       ::= <statement> | <declaration>
<statement>   ::= "return" <exp> ";" | <expr> ";" | ";" | <identifier> ":" <statement> | "goto" <identifier> ";"
                | "if" "(" <expr> ")" <statement> [ "else" <statement> ]
<declaration> ::= "int" <identifier> [ "=" <expr> ] ";"
<expr>        ::= <factor> | <expr> <binop> <expr> | <expr> "?" <expr> ":" <expr>
<factor>      ::= <int> | <identifier> | <unop> <factor> | "(" <expr> ")" 
<unop>        ::= "-" | "~" | "!" | <increment>
<increment>   ::= "++" | "--"
<identifier>  ::= ? An identifier token ?
<int>         ::= ? A constant token ?
<binop>       ::= "+" | "-" | "*" | "/" | "%" | "&" | "|" | "^" | "<<" | ">>" | "&&" | "||" | "==" | "!=" 
                | "<" | "<=" | ">" | ">=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>"
```

