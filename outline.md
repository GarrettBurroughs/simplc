# C Compiler in rust 

*Goal:* Implement a subset of the C programming language in a Rust Compiler

# Lexer 

*Goal:* Lex the following program 

int main() {
    return 1;
}

# Parser

<program> ::= <function>
<function> ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
<statement> ::= "return" <exp> ";"
<exp> ::= <int>
<identifier> ::= ? An identifier token ?
<int> ::= ? A constant token ?

