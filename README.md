## Rust programming

- Rust Language https://www.rust-lang.org
- Rustup (Installer fo Rust) https://rustup.rs
- GDB (The GNU Project Debugger) https://www.sourceware.org/gdb
- GEF (GDB Enhanced Features) https://github.com/hugsy/gef

### Started Rust programming

- Intall Rust Programming [here](https://www.rust-lang.org/tools/install) `https://www.rust-lang.org/tools/install`
- **Linux or macOS**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Check rust installationed
rustc --version

## Unistall Rust
rustup self uninstall
```

- **Windows**

```bash
https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
```

- **Fix Cargo Expand**

```bash
rustup install nightly
```

### Init Rust project using cargo

```bash
cargo new <project_name>
```

### Compliler normal rust

-

```bash
rustc main.rs
```

### Compiler Rust using Cargo

```bash
# Dev
cargo check ## It's only compiler
cargo build ## compiler and build binary file
cargo run  ## build binary file and running in target folder

# Production
cargo build --release # Build For production
```

### Crate Tools

- cargo-edit https://crates.io/crates/cargo-edit
- cargo-expand https://crates.io/crates/cargo-expand

### Crate Package

- rocket https://crates.io/crates/rocket
- crossterm https://crates.io/crates/crossterm
- rusty_audio https://crates.io/crates/rusty_audio
- rusty_time https://crates.io/crates/rusty_time
