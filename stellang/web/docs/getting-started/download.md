---
sidebar_position: 1
---

# Download

## Git clone (recommended)

Step 1: Make sure to have `git` and `rust` installed

```bash
git --version
cargo --version
```

**NOTE**: To install `Rust` with `Cargo`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Step 2: Git clone the Crabby repository

```bash
git clone https://github.com/crabby-lang/crabby.git
```

Step 3: Build it

```bash
cargo build
```

Step 4: Test and Run it

```bash
cargo run ./examples/example.crab
```

## System Requirements

* Windows 10 (64-bit) or later
* 2-4 GB RAM or more
* 100MB Disk Space or more

## Additional Notes

* An internet connection is required for initial download and potential updates
* Crabby works only in the Rust nightly version
* This page might changed for the upcoming updates
