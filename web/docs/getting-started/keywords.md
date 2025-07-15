---
authors: [Mahesh]
sidebar_position: 1
---

# StelLang Keywords

**By Mahesh**

Here’s a quick reference to the essential keywords in StelLang, with modern examples and explanations.

## pub
Marks a function as public (importable):
```stel
pub def greet() {
    print("Hello!")
}
```

## import
Imports a module or file:
```stel
import greet from "./greet.stel"
greet()
```

## def
Defines a function:
```stel
def add(a, b) {
    return a + b
}
```

## let
Declares a variable:
```stel
let x = 10
```

## if / else
Conditional logic:
```stel
let x = true
if x {
    print("True!")
} else {
    print("False!")
}
```

## docstrings
Add documentation to functions:
```stel
def foo() {
    """
    This is a docstring!
    """
    print("Hello!")
}
```

## decorators (EXPERIMENTAL)
Modify or extend function behavior:
```stel
def log() {
    print("Logging!")
}

@log
def run() {
    print("Running!")
}
```

---

*See the [full docs](/docs/stellang) for more language features!*
