# Contributing to via

Thank you for your interest in contributing to the **via programming language**!  
We welcome issues, pull requests, ideas, and feedback.

---

## Getting Started

1. **Fork the repository** and clone your fork:
    ```bash
    git clone https://github.com/via-lang/via.git
    cd via-lang
    ```

2. **Build the project**:
    ```bash
    cargo build --release
    ```
    Or test:
    ```bash
    cargo test
    ```

---

## Where to Contribute

- 🧠 **Bytecode Interpreter**: Instruction set, execution model, memory system
- 📦 **Compiler/Parser**: Frontend logic, AST, bytecode generation
- 🧹 **Compiler UX**: Improvements to diagnostics, CLI, etc.
- 📖 **Docs**: Clarifications, grammar rules, developer notes
- 🧪 **Tests**: Add minimal reproducible tests for language features
- 🧩 **Standard Library**: Extend the standard library of the language

---

## Code Style

- Comply with the [Rust syle guide](https://doc.rust-lang.org/beta/style-guide/index.html).
- Use [clippy](https://doc.rust-lang.org/stable/clippy/usage.html).
- Avoid [dependency hell](https://en.wikipedia.org/wiki/Dependency_hell).
- Avoid the following Rust features unless absolutely necessary (with the exception of `via_vm`):
    - Requires `unsafe {}`:
        - Dereferencing raw pointers
        - Calling `unsafe` functions or FFI
        - Accessing `static mut`
        - Implementing `unsafe` traits
        - Accessing `union` fields
    - Panics (safe but discouraged):
        - `.unwrap()` / `.expect()` — use `?` or explicit matching
        - Direct indexing `slice[i]` — use `.get(i)`
        - Integer overflow (wraps silently in release builds)
        - `panic!()` / `todo!()` / `unreachable!()` in non-stub code

---

## Pull Request Guidelines

- Open a draft PR early if unsure, discussion is welcome.
- Keep PRs focused and minimal. One feature/fix per PR is ideal.
- Include comments for complex logic.
- If you add a new feature, add a minimal test case if possible.

---

## Reporting Issues

- Use the [Issues](https://github.com/via-lang/via/issues) tab.
- Include:
  - What you expected vs. what happened
  - Minimal reproducible example
  - Version/commit hash if possible

---

## Licensing

All contributions must be compatible with the [GNU GPL v3](./LICENSE).

---

Thanks again for helping make via better!
