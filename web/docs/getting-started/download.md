---
authors: [Mahesh]
sidebar_position: 1
---

# Download & Install StelLang

**By Mahesh**

Get started with StelLang in just a few steps!

## Prerequisites
- [Git](https://git-scm.com/)
- [Rust (nightly)](https://www.rust-lang.org/tools/install)

## Installation Steps

1. **Check your tools:**
   ```sh
   git --version
   cargo --version
   ```
2. **Install Rust (if needed):**
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup default nightly
   ```
3. **Clone the StelLang repository:**
   ```sh
   git clone https://github.com/MaheshDhingra/StelLang
   cd StelLang
   ```
4. **Build StelLang:**
   ```sh
   cargo build
   ```
5. **Run your first program:**
   ```sh
   cargo run ./examples/example.stel
   ```

## System Requirements
- Windows 10 (64-bit) or later
- 2-4 GB RAM or more
- 100MB disk space or more

## Notes
- Internet required for initial setup and updates
- StelLang requires Rust nightly

---

*For help, join the [community](/community) or see the [full documentation](/docs/stellang).*
