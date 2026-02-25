# SimplC Grammar

```
<program>                 ::= { <function-declaration> }
<declaration>             ::= <function-declaration> | <variable-declaration>
<function-declaration>    ::= "int" <identifier> "(" <param-list> ")" ( <block> | ";" )
<variable-declaration>    ::= "int" <identifier> [ "=" <expr> ] ";"
<block>                   ::= "{" { <block-item> } "}"
<block-item>              ::= <statement> | <declaration>
<init>                    ::= <variable-declaration> | [ <expr> ] ";"
<statement>               ::= "return" <exp> ";" 
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
<param-list>              ::= "void" | "int" <identifier> { "," "int" <identifier> }
<expr>                    ::= <factor> | <expr> <binop> <expr> | <expr> "?" <expr> ":" <expr>
<factor>                  ::= <int> | <identifier> | <unop> <factor> | "(" <expr> ")" 
                            | <identifier> "(" [ <argument-list> ] ")"
<unop>                    ::= "-" | "~" | "!" | <increment> 
<argument-list>           ::= <expr> { "," <expr> }
<increment>               ::= "++" | "--"
<identifier>              ::= ? [a-zA-Z_]\w*\b ?
<int>                     ::= ? [0-9]+\b ?
<binop>                   ::= "+"  | "-"  | "*"  | "/"  | "%" 
                            | "&"  | "|"  | "^" 
                            | "<<" | ">>" 
                            | "&&" | "||" | "==" | "!=" 
                            | "<"  | "<=" | ">"  | ">=" 
                            | "+=" | "-=" | "*=" | "/="  | "%=" 
                            | "&=" | "|=" | "^=" | "<<=" | ">>="
```

