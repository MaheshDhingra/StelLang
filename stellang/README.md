# StelLang

A fast, expressive, Rust-powered programming language designed for engineers.

## Features
- Python-like syntax, Rust-like performance
- Variables, arithmetic, and assignment
- Control flow: `if`, `else`, `while`, `break`, `continue`
- Functions and function calls
- Blocks and scopes
- Built-in `print` function
- Package manager: `pico`
- Logical operators: `and`, `or`, `not`

## Getting Started

### 1. Build the Language
```sh
cargo build
```

### 2. Run the REPL
```sh
cargo run
```

### 3. Run a Script
Write your StelLang code in `src/main.stl` and run:
```sh
cargo run
```

## Language Examples

### Variables and Arithmetic
```stel
x = 10
y = 20
z = x * 2 + y / 5
print(z)
```

### If/Else
```stel
if z > 20 {
    print("Large value!")
} else {
    print("Small value!")
}
```

### While Loop
```stel
count = 0
while count < 5 {
    print(count)
    count = count + 1
}
```

### Functions
```stel
fn add(a, b) {
    return a + b
}
result = add(3, 4)
print(result)
```

### Break and Continue
```stel
count = 0
while count < 10 {
    if count == 3 {
        count = count + 1
        continue
    }
    if count == 7 {
        break
    }
    print(count)
    count = count + 1
}
```

### Logical Operators
```stel
a = 1
b = 0
if a and not b {
    print("a is true and b is false!")
}
if a or b {
    print("at least one is true!")
}
```

## Package Manager: pico

### Initialize a Project
```sh
cargo run --bin pico -- init
```

### Add a Dependency
```sh
cargo run --bin pico -- add
```

### Build the Project
```sh
cargo run --bin pico -- build
```

### Install Dependencies
```sh
cargo run --bin pico -- install
```

### Publish a Package
```sh
cargo run --bin pico -- publish
```

## Testing Language Features

### 1. Arithmetic and Assignment
- Enter `x = 5 + 2 * 3` in the REPL. Output should be `= 11`.
- Enter `print(x)` to print the value.

### 2. If/Else
- Enter:
  ```stel
  if x > 10 {
      print(1)
  } else {
      print(0)
  }
  ```
- Output should be `1` if `x > 10`, else `0`.

### 3. While Loop
- Enter:
  ```stel
  count = 0
  while count < 3 {
      print(count)
      count = count + 1
  }
  ```
- Output should be `0`, `1`, `2` on separate lines.

### 4. Functions
- Enter:
  ```stel
  fn square(n) { return n * n }
  print(square(5))
  ```
- Output should be `25`.

### 5. Print
- Enter `print(123)` or `print(x)` to print values.

### 6. Scripts
- Edit `src/main.stl` with any StelLang code and run `cargo run` to execute it.

## Contributing
Pull requests and feedback are welcome!

---
*Note: This language is under active development. Features and syntax may change.*
