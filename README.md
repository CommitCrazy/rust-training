# Rust Training

Practice projects from the [Microsoft Rust Training](https://github.com/microsoft/RustTraining) course — a hands-on guide to learning Rust for developers coming from other languages.

Each chapter's exercises are stored as separate Cargo projects in this repo.

## Projects

| Project | Chapter | Topics |
|---------|---------|--------|
| [hello_rust](./hello_rust) | [Ch 2 — Getting Started](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch02-getting-started.md) | Variables, mutability, loops, pattern matching |
| [temperature_converter](./temperature_converter) | [Ch 3 — Built-in Types and Variables](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch03-built-in-types-and-variables.md) | Floating-point types (f64), functions, match guards, &'static str, string formatting |
| [fizzbuzz_with_expressions](./fizzbuzz_with_expressions) | [Ch 4 — Control Flow](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch04-control-flow.md) | For loops, range expressions, pattern matching (match), tuple destructuring, wildcard patterns |
| [word_frequency_counter](./word_frequency_counter) | [Ch 5 — Data Structures and Collections](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch05-data-structures-and-collections.md) | HashMap, entry API, iterators, split_whitespace, String vs &str, mutable references |
| [shape_area_calculator](./shape_area_calculator) | [Ch 6 — Enums and Pattern Matching](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch06-enums-and-pattern-matching.md) | Enums with struct-like variants, impl blocks, methods, match pattern matching, variant destructuring, std::f64::consts::PI |
| [spot_the_borrow_checker_error](./spot_the_borrow_checker_error) | [Ch 7 — Ownership and Borrowing](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch07-ownership-and-borrowing.md) | Ownership, immutable/mutable borrows, references, non-lexical lifetimes (NLL), &str vs String, move semantics |
| [module_visibility](./module_visibility) | [Ch 8 — Crates and Modules](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch08-crates-and-modules.md) | Modules (`mod`), visibility (`pub`), nested submodules, `super::` paths, path resolution (`::`), private-by-default encapsulation |

## Prerequisites

- [Rust toolchain](https://rustup.rs/) (rustc, cargo)

## Running a project

```sh
cd hello_rust
cargo run
```
