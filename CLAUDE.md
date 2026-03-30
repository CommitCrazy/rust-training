# Rust Training

This repo contains practice projects from the [Microsoft Rust Training](https://github.com/microsoft/RustTraining) course (python-book track).

## Structure

Each chapter's exercises live in their own Cargo project at the repo root (e.g., `hello_rust/`).

## Commit & Push Workflow

When the user asks to commit and push a new or updated chapter project:

1. **Update `README.md`** — Add or update the row in the Projects table matching the project being committed. Each row follows this format:
   ```
   | [project_name](./project_name) | [Ch N — Title](https://github.com/microsoft/RustTraining/blob/main/python-book/src/chNN-slug.md) | Comma-separated topics |
   ```
   - Infer the chapter number and title from the project code and the course structure.
   - List the key Rust topics/concepts covered in the project.
2. **Commit** all changed files (the project + updated README) together in a single commit.
3. **Push** to origin.

## Course Chapter Reference

The course chapters live at `https://github.com/microsoft/RustTraining/blob/main/python-book/src/` with filenames like `ch02-getting-started.md`, `ch03-types.md`, etc.
