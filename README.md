<h1 align="center">
    <br>
        Cargo Dev
    <br>
</h1>
<h4 align="center">
    Alias shell commands (think NPM scripts for cargo)
</h4>
<p align="center">
    <a href="https://github.com/MNThomson/cargo-dev/commits">
        <img
            src="https://img.shields.io/github/last-commit/MNThomson/cargo-dev?style=flat-square"
            alt="Last GitHub Commit"
        >
    </a>
    <a href="https://crates.io/crates/cargo-dev">
        <img
            src="https://img.shields.io/crates/d/cargo-dev?style=flat-square"
            alt="Last GitHub Commit"
        >
    </a>
    <a href="https://crates.io/crates/cargo-dev">
        <img
            src="https://img.shields.io/crates/v/cargo-dev?style=flat-square"
            alt="Last GitHub Commit"
        >
    </a>
    
</p>

---

## Install

```bash
cargo install cargo-dev
```

## Usage

Add a `[dev]` table (section) to your `Cargo.toml` with aliases to shell commands

```toml
[dev]
sayhi = "echo Hello World"
```

Running `cargo dev sayhi`:

```bash
$ cargo dev sayhi
> sayhi: echo Hello World
Hello World
```

## Alpha Notice

`cargo-dev` is currently in early Alpha. Branch `master` is not guaranteed to be stable and breaking changes may be introduced without notice.
