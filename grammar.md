# SimplC Grammar

```
<program>     ::= <function>
<function>    ::= "int" <identifier> "(" "void" ")" <block>
<block>       ::= "{" { <block-item> } "}"
<block-item>  ::= <statement> | <declaration>
<init>        ::= <declaration> | [ <expr> ] ";"
<statement>   ::= "return" <exp> ";" 
                | <expr> ";" 
                | "if" "(" <expr> ")" <statement> [ "else" <statement> ]
                | <identifier> ":" <statement> 
                | "goto" <identifier> ";"
                | <block>
                | ";" 
                | "while" "(" <expr> ")" <statement> 
                | "do" <statement> "while" "(" <expr> ")" ";"
                | "for" "(" <init> [ <expr> ]; [ <expr> ] ")" <statement> 
                | "switch" "(" <expr> ")" <statement>
                | "case" <expr> ":" <statement>
                | "default" ":" <statement>
                | "break" ";"
                | "continue" ";"
<declaration> ::= "int" <identifier> [ "=" <expr> ] ";"
<expr>        ::= <factor> | <expr> <binop> <expr> | <expr> "?" <expr> ":" <expr>
<factor>      ::= <int> | <identifier> | <unop> <factor> | "(" <expr> ")" 
<unop>        ::= "-" | "~" | "!" | <increment>
<increment>   ::= "++" | "--"
<identifier>  ::= ? [a-zA-Z_]\w*\b ?
<int>         ::= ? [0-9]+\b ?
<binop>       ::= "+"  | "-"  | "*"  | "/"  | "%" 
                | "&"  | "|"  | "^" 
                | "<<" | ">>" 
                | "&&" | "||" | "==" | "!=" 
                | "<"  | "<=" | ">"  | ">=" 
                | "+=" | "-=" | "*=" | "/="  | "%=" 
                | "&=" | "|=" | "^=" | "<<=" | ">>="
```

