# xxx_rust_compiler
xxx: a C-to-Rust converter and compiler and gcc replacement

# Full Rust C Compiler

This project is a minimal Rust-based compiler that parses full C11 source code using the `lang-c` crate and compiles it into raw object files via Cranelift. It serves as a lightweight alternative to GCC for simple C programs.

## Features

- Parses full C11 code using `lang-c`
- Generates object files using Cranelift
- Supports basic `return` statements

## Requirements

- Rust (edition 2021)
- `lang-c` crate
- Cranelift crates

## Usage

1. Build the project:

   ```bash
   cargo build --release

2. Compile a C source file:
./target/release/xxx_rust_compiler input.c output.o
Limitations

Currently supports only simple return statements.
Does not handle complex C constructs or standard library functions.
License

This project is dual-licensed under the MIT and Apache 2.0 licenses.

:contentReference[oaicite:33]{index=33}

---

:contentReference[oaicite:35]{index=35} :contentReference[oaicite:38]{index=38} :contentReference[oaicite:41]{index=41}:contentReference[oaicite:43]{index=43}
::contentReference[oaicite:44]{index=44}
   
