# StelLang

A fast, expressive, Rust-powered programming language designed for engineers, with a Pythonic feel and Rust-like performance.

---

## 🚀 Getting Started

### 1. Build the Language
```sh
cargo build
```

### 2. Run the REPL
```sh
cargo run --bin stellang
```

### 3. Run a Script
Write your StelLang code in any `.stel` file, e.g. `main.stel`:
```sh
cargo run --bin stellang -- main.stel
```

---

## 🧪 Running All Tests

Run the full test suite (unit, integration, and language tests):
```sh
cargo test
```

Run a specific test file:
```sh
cargo test --test operator_tests
cargo test --test builtin_method_tests
```

To add new tests, see the `tests/` directory for examples. Add your `.rs` or `.stel` test files and use Rust’s test framework or language-level assertions.

---

## 📦 Package Manager: `stel`

StelLang comes with a built-in package manager and CLI tool called `stel`.

### Registry
- The default registry is: **https://stellang.maheshdhingra.xyz/registry**
- You can publish, install, and search for packages here.

### Common Commands

#### Initialize a Project
```sh
cargo run --bin stel -- init
```

#### Add a Dependency
```sh
cargo run --bin stel -- add <package>[@<version>]
```

#### Build the Project
```sh
cargo run --bin stel -- build
```

#### Install Dependencies
```sh
cargo run --bin stel -- install
```

#### Run Tests
```sh
cargo run --bin stel -- test
```

#### Update Dependencies
```sh
cargo run --bin stel -- update
```

#### Publish a Package
```sh
cargo run --bin stel -- publish
```

#### Search the Registry
```sh
cargo run --bin stel -- search <query>
```

#### Run a Script
```sh
cargo run --bin stel -- run <file.stel>
```

#### Clean Build Artifacts
```sh
cargo run --bin stel -- clean
```

#### Get Help
```sh
cargo run --bin stel -- help
```

---

## 🧪 Testing the Package Manager

Tests for the `stel` package manager are in `tests/` and cover:
- Project initialization
- Adding, installing, and updating dependencies
- Building and running projects
- Registry search and publish
- Error handling (network, version, etc.)

To run all package manager tests:
```sh
cargo test --test package_manager_tests
```

To add new tests, create a file like `tests/package_manager_tests.rs` and use Rust’s test framework. You can also add `.stel` scripts in `tests/` and invoke them via the CLI in your tests.

---

## 📝 Language Features

- Python-like syntax, Rust-like performance
- Variables, arithmetic, assignment
- Control flow: `if`, `else`, `while`, `break`, `continue`
- Functions, blocks, scopes
- Built-in types: int, float, str, list, dict, set, tuple, bytes, bytearray, range, etc.
- Slicing, iteration, comprehensions (WIP)
- Pattern matching: `match`, `case`
- Exception system: Python-style exceptions, try/catch, throw
- Import/module system
- Package manager: `stel`

See `tests/` for feature tests and usage examples.

---

## 🤝 Contributing

1. Fork the repo and clone it.
2. Create a new branch for your feature or bugfix.
3. Write code and **add tests** in `tests/`.
4. Run `cargo test` and ensure all tests pass.
5. Submit a pull request with a clear description.

All contributions should include tests and documentation updates as needed.


---

## 🌐 Registry

- Default registry: [https://stellang.maheshdhingra.xyz/registry](https://stellang.maheshdhingra.xyz/registry)
- You can publish and search for packages here using the `stel` CLI.

---
