---
title: Stel Package Manager (stel.rs)
authors: [Mahesh]
sidebar_position: 1
---

# Stel Package Manager: `stel.rs`

**Author: Mahesh**

The Stel package manager, `stel.rs`, is a powerful tool for managing, building, and publishing StelLang projects and libraries. Inspired by Cargo, it is optimized for the StelLang ecosystem and provides a fast, intuitive workflow for developers.

## Why Use stel.rs?

- **Project Initialization**: Quickly scaffold new StelLang projects.
- **Dependency Management**: Add, update, and remove dependencies with ease.
- **Building & Running**: Compile and run your StelLang code efficiently.
- **Testing**: Run tests to ensure code quality.
- **Documentation**: Generate and view project documentation.
- **Publishing**: Share your packages with the community via the Stel Registry.

## Installation

`stel.rs` is included when you build StelLang from source. See the [Download & Install Guide](./getting-started/download.md) for details.

## Common Commands

| Command            | Description                                 |
|--------------------|---------------------------------------------|
| `stel init`        | Initialize a new Stel project               |
| `stel build`       | Build the current project                   |
| `stel run`         | Run the current project                     |
| `stel test`        | Run tests in the project                    |
| `stel doc`         | Generate documentation                      |
| `stel publish`     | Publish the project to the Stel Registry    |
| `stel install`     | Install a package from the registry         |
| `stel add <pkg>`   | Add a dependency to your project            |

## Example Usage

### Initialize a Project
```sh
stel init my_project
cd my_project
```

### Add a Dependency
```sh
stel add example-http
```

### Build and Run
```sh
stel build
stel run
```

### Publish to Registry
```sh
stel publish
```

## Registry

The default registry is: [https://stellang.maheshdhingra.xyz/registry](https://stellang.maheshdhingra.xyz/registry)

## More Information

- [StelLang Overview](./stellang.md)
- [Official Registry](https://stellang.maheshdhingra.xyz/registry)
- [GitHub Repository](https://github.com/MaheshDhingra/StelLang)

---

*This documentation is maintained by Mahesh. Contributions and feedback are welcome!* 