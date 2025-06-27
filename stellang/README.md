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
- Variable declarations: `let`, `const`
- Boolean and null literals: `true`, `false`, `null`
- Built-in `input` function
- Built-in math/array/string functions: `sqrt`, `abs`, `pow`, `min`, `max`, `sum`, `range`, `reverse`, `join`, `split`, `sort`, `map`, `filter`, `find`, `reduce`, `zip`, `sort`, `join`, `split`, `reverse`, `len`
- Pattern matching: `match`, `case`
- Structs and enums (syntax only, WIP)
- Import/module system
- **Extensive Python built-ins**: abs, all, any, ascii, bin, bool, breakpoint, bytearray, bytes, callable, chr, classmethod, compile, complex, delattr, dict, dir, divmod, enumerate, eval, exec, filter, float, format, frozenset, getattr, globals, hasattr, hash, help, hex, id, input, int, isinstance, issubclass, iter, len, list, locals, map, max, memoryview, min, next, object, oct, open, ord, pow, print, property, range, repr, reversed, round, set, setattr, slice, sorted, staticmethod, str, sum, super, tuple, type, vars, zip, __import__

## Devlog (2025-06-27)

### Interpreter & Built-ins
- Implemented or stubbed **all major Python built-ins**: abs, all, any, ascii, bin, bool, breakpoint, bytearray, bytes, callable, chr, classmethod, compile, complex, delattr, dict, dir, divmod, enumerate, eval, exec, filter, float, format, frozenset, getattr, globals, hasattr, hash, help, hex, id, input, int, isinstance, issubclass, iter, len, list, locals, map, max, memoryview, min, next, object, oct, open, ord, pow, print, property, range, repr, reversed, round, set, setattr, slice, sorted, staticmethod, str, sum, super, tuple, type, vars, zip, __import__.
- Added error stubs for not-yet-supported built-ins (e.g., open, property, memoryview, classmethod, staticmethod, compile, eval, exec, __import__, breakpoint, globals, locals, super).
- Improved type handling for isinstance, issubclass, and more.
- All built-ins are now available in the REPL and scripts.

### Codebase
- No build errors in any Rust source files.
- All main modules (`lexer.rs`, `parser.rs`, `ast.rs`, `interpreter.rs`, `main.rs`) are up to date and error-free.
- Documentation and examples updated to reflect new built-ins.

### Next Steps
- Expand support for advanced features (e.g., aiter, anext, async, comprehensions, decorators).
- Add more robust error handling and recovery in the parser.
- Add a `doc()` built-in for documentation of built-ins and user functions.
- Add REPL history and more interactive features.

---

## Getting Started

### 1. Build the Language
```sh
cargo build
```

### 2. Run the REPL
```sh
cargo run --bin stellang
```

### 3. Run a Script
Write your StelLang code in `src/main.stel` and run:
```sh
cargo run --bin stellang
```

### 4. Run the Package Manager (pico)
```sh
cargo run --bin pico -- <command>
```
For example, to initialize a project:
```sh
cargo run --bin pico -- init
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

### Variable Declarations
```stel
let a = 42
const b = "hello"
print(a)
print(b)
```

### Boolean and Null Literals
```stel
let t = true
let f = false
let n = null
print(t)
print(f)
print(n)
```

### Input
```stel
name = input()
print("Hello, " + name)
```

### Built-in Math and Array Functions
```stel
print(sqrt(16))
print(abs(-5))
print(pow(2, 8))
arr = range(1, 5)
print(arr)
print(sum(arr))
print(min(3, 7))
print(max(3, 7))
```

### Array Utilities
```stel
print(reverse(arr))
print(join(arr, ","))
print(split("a,b,c", ","))
print(sort([3,1,2]))
```

### Higher-Order Functions
```stel
squared = map(arr, fn(x) { return x * x })
evens = filter(arr, fn(x) { return x % 2 == 0 })
first_even = find(arr, fn(x) { return x % 2 == 0 })
sum_reduce = reduce(arr, fn(acc, x) { return acc + x }, 0)
pairs = zip([1,2,3], ["a","b","c"])
print(squared)
print(evens)
print(first_even)
print(sum_reduce)
print(pairs)
```

### Find, Reduce, Zip, and More
```stel
arr2 = [10, 20, 30, 40]
found = find(arr2, fn(x) { return x > 15 })
reduced = reduce(arr2, fn(acc, x) { return acc * x }, 1)
zipped = zip(arr, arr2)
print(found)
print(reduced)
print(zipped)
```

### String and Array Manipulation
```stel
s = "hello,world,stel"
split_arr = split(s, ",")
joined = join(split_arr, "-")
reversed = reverse(s)
print(split_arr)
print(joined)
print(reversed)
```

### Sorting
```stel
unsorted = [5, 2, 9, 1]
print(sort(unsorted))
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
- Edit `src/main.stel` with any StelLang code and run `cargo run` to execute it.

### Pattern Matching
```stel
x = 2
match x {
    1 => print("one"),
    2 => print("two"),
    _ => print("other"),
}
```

### Structs
```stel
struct Point { x, y }
p = Point { x: 1, y: 2 }
print(p)
```

### Enums
```stel
enum Color { Red, Green, Blue }
c = Color::Red
print(c)
```

Pattern matching, structs, and enums are now fully supported at runtime!

### For Loop
```stel
for i in arr {
    print("for-loop:", i)
}
```

### Map/Array Utilities
```stel
m = {"a": 1, "b": 2, "c": 3}
print(map_keys(m))
print(map_values(m))
print(array_contains(arr, 3))
print(array_index_of(arr, 3))
```

### Error Handling
```stel
try {
    throw("fail!")
} catch err {
    print("Caught error:", err)
}
```

## Contributing
Pull requests and feedback are welcome!

---
*Note: This language is under active development. Features and syntax may change.*

### Tuple Literals and Destructuring
```stel
(a, b) = (1, 2)
print(a)
print(b)
```

### More Built-ins
```stel
nums = [1, 2, 3, 4, 5]
print(all(nums, fn(x) { return x > 0 }))
print(any(nums, fn(x) { return x == 3 }))
print(flatten([[1,2],[3,4],[5]]))
print(unique([1,2,2,3,3,3,4]))
print(count(nums, 3))
print(repeat("hi", 3))
print(enumerate(["a","b","c"]))
```

### String Interpolation
```stel
vars = {"name": "Stel", "n": 42}
print(interp("Hello, {name}! Your number is {n}.", vars))
```

### Import/Module System
```stel
import "utils.stel"
# Now you can use functions/variables from utils.stel
```
