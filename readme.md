# SimplC Compiler

SimplC is a toy compiler for a subset of the C programming language. Built in Rust, it uses **LLVM** (via the [Inkwell](https://github.com/TheLobster/inkwell) library) for code generation. This project is primarily designed for educational purposes, helping to explore compiler frontend design, semantic validation passes, and LLVM-based code generation.

---

## Table of Contents
1. [Project Overview](#project-overview)
2. [Project Structure](#project-structure)
3. [Language Features](#language-features)
4. [Compiler Pipeline](#compiler-pipeline)
5. [Prerequisites](#prerequisites)
6. [Building and Running](#building-and-running)
7. [AST Visualization](#ast-visualization)
8. [Logging and Debugging](#logging-and-debugging)
9. [Roadmap](#roadmap)

---

## Project Overview

SimplC parses a subset of the C programming language and outputs LLVM IR (`.ll`) and target-native assembly (`.s`). By linking the emitted assembly using an external compiler like `gcc` or `clang`, it compiles C programs down to native executables.

---

## Project Structure

The project codebase is modularly organized as follows:

*   **`src/`**
    *   **`frontend/`** Lexing Parsing and AST generation
    *   **`semantic/`** semantic passes (variable resolution, loop labeling, etc)
    *   **`codegen/`** llvm backend
*   **`examples/`** example files used for exploring and testing the compiler
*   **`scripts/`** files for running and visualizing compiler output
*   **[grammar.md](./grammar.md)** Describes the BNF-style grammar rules supported by the compiler.

---

## Language Features

SimplC supports a robust subset of C:
*   **Data Types**: Currently only support `int` and `void` parameter lists.
*   **Variable Declarations & Scopes**: Block-scoped local variables and functions. Fully supports nested scopes and variable shadowing (renaming shadowed local variables).
*   **Functions**: Declaring functions (e.g., `int main(void)`), calling functions, and passing arguments.
*   **Control Flow**:
    *   `if` / `else` conditional execution.
    *   `while` and `do-while` loops.
    *   `for` loops (supports declarations in initializers).
    *   `goto` with label statements.
    *   `switch` / `case` / `default` selection blocks.
    *   `break` and `continue` control jumps.
*   **Expressions & Operators**:
    *   *Arithmetic*: `+`, `-`, `*`, `/`, `%`
    *   *Bitwise*: `&`, `|`, `^`, `<<`, `>>`, `~`
    *   *Logical*: `&&`, `||`, `!`
    *   *Comparison*: `==`, `!=`, `<`, `<=`, `>`, `>=`
    *   *Assignment*: `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`
    *   *Ternary (Conditional)*: `? :`
    *   *Unary*: prefix and postfix `++` and `--`

---

## Compiler Pipeline

The compilation process is split into several stages:
1.  **Preprocessing**: The input C code is preprocessed using an external tool (e.g., `gcc -E`) to strip comments and expand macros.
2.  **Lexical Analysis (Lexer)**: The character stream of the preprocessed file is scanned into structured tokens.
3.  **Parsing (Parser)**: The token stream is parsed using recursive descent into an Abstract Syntax Tree (AST).
4.  **Semantic Analysis**:
    *   *Variable Resolution*: Checks variables, binds them to local scopes, and uniquely renames them to resolve shadowing.
    *   *Label Resolution*: Ensures that `goto` statements target existing labels and that label names are unique.
    *   *Loop/Control Flow Binding*: Maps `break`/`continue` statements and `case`/`default` statements to their valid parents.
5.  **Code Generation (LLVM)**: The AST is traversed to generate LLVM IR, which is optimized and compiled into native assembly (`.s`).
6.  **Linking (External)**: The output assembly is linked by GCC or Clang to build the native binary.

---

## Prerequisites

To build and run SimplC, ensure you have the following installed on your system:
*   **Rust (Edition 2024)**: Install via [rustup](https://rustup.rs/).
*   **LLVM 18**: The `inkwell` crate requires the LLVM 18 development libraries.
    *   *Ubuntu/Debian*: `sudo apt-get install llvm-18-dev libclang-18-dev`
*   **GCC / Clang**: For preprocessing and final linking.
*   **Python 3 & Graphviz** (Optional): Required for AST visualization.
    *   Python dependencies: `pip install graphviz`

---

## Building and Running

### 1. Build the Compiler
Build the Rust binary using Cargo:
```bash
cargo build --release
```
This generates the executable inside `target/release/compiler` (or `target/debug/compiler` if built without `--release`).

### 2. Using the Compiler CLI
You can run the compiler directly with:
```bash
cargo run -- <input_file.i> [options]
```
#### CLI Options:
*   `-l`, `--lex`: Output the lexical analysis results (tokens list) to `<output_name>.lex`.
*   `--ast`: Output the textual AST representation to `<output_name>.ast`.
*   `--ir`: Output the generated LLVM IR to `<output_name>.ll`.
*   `--asm`: Output target-native assembly (written to `<output_name>.s` automatically).
*   `-o`, `--output <path>`: Specify the output file prefix/location.
*   `--debug-level <level>`: Specify log filter level (`off`, `error`, `warn`, `info`, `debug`, `trace`).

### 3. Automated End-to-End Execution
You can use the helper driver script inside `scripts/` to preprocess, compile, link, and run a C source file in one command:
```bash
# Using driver2.sh to compile and run an example
./scripts/driver2.sh examples/operators.c
```
*(Note: You may need to edit the hardcoded compiler path inside the script if your local directories differ.)*

### 4. Manual End-to-End Pipeline
To compile a C file step-by-step:
```bash
# 1. Preprocess using GCC
gcc -E -P examples/operators.c -o examples/operators.i

# 2. Compile preprocessed file to assembly (.s) and generate IR/AST
cargo run -- examples/operators.i --ast --ir

# 3. Assemble and link with GCC
gcc examples/operators.s -o examples/operators

# 4. Execute the binary
./examples/operators
echo $? # Prints the return value of main
```

---

## AST Visualization

The compiler outputs a textual structure showing AST node indentations when using the `--ast` flag. You can render this into a graphical tree diagram using the Python script and Graphviz:
```bash
# 1. Generate the .ast file
cargo run -- examples/operators.i --ast

# 2. Run the visualizer python script
python3 scripts/visualizer.py examples/operators.ast
```
This will render and save a PNG file named `compiler_ast.png` visualizing the AST.

---

## Logging and Debugging

The compiler uses `env_logger` and the `log` crate to print internal compilation metrics and steps. Set the `--debug-level` flag to control log verbosity:
*   `error`: Shows errors.
*   `warn`: Shows compiler warnings.
*   `info`: Shows compiler stages (Lexing, Parsing, Semantic passes, CodeGen).
*   `debug`: Shows detailed compiler constructs (Dumped Tokens, AST structure, LLVM IR).
*   `trace`: Very verbose diagnostic information.

---

## Roadmap

*   [ ] Treat functions as declarations
*   [ ] Implement a type checker (for functions)
