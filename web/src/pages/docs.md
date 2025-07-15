---
title: StelLang Examples & Features
authors: [Mahesh]
sidebar_position: 2
---

# StelLang: Features & Examples

**By Mahesh**

Explore the power and simplicity of StelLang with these up-to-date features and code examples.

## Key Features

- **Simple, Pythonic Syntax**: Easy to read and write.
- **Functional & Procedural**: Mix paradigms as needed.
- **Async/Await**: Write non-blocking code with ease.
- **Pattern Matching**: Powerful control flow for complex data.
- **Gradual Typing**: Choose static or dynamic typing.
- **Lambdas & Decorators**: Expressive, modern constructs.
- **Robust Exception System**: Handle errors gracefully.
- **Built-in Package Manager**: Manage dependencies with `stel.rs`.

## Examples

### Hello World
```stel
print("hello world!")
```

### Math
```stel
let x = 10
let y = 20
print(x + y)
```

### If-Else
```stel
let x = true
if x {
    print("True!")
} else {
    print("False!")
}
```

### Lambda
```stel
let add = lambda(a, b) {
    return a + b
}
print(add(5, 7))
```

### Async/Await
```stel
async def fetch_data() {
    // Simulate async work
    return 42
}
let result = await fetch_data()
print(result)
```

### Pattern Matching
```stel
let value = 7
match value {
    0 => print("Zero"),
    1..10 => print("Between 1 and 10"),
    _ => print("Other")
}
```

### Gradual Typing
```stel
def add(a: int, b: int) -> int {
    return a + b
}
print(add(2, 3))
```

---

*See the [full documentation](/docs/stellang) for more!*
