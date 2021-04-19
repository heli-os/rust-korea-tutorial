## 20210419

### Getting Started

```shell
rustc main.rs
```

### Cargo

#### Creating a Project with Cargo

```shell
cargo new hello_cargo --vcs=git
```

#### Building and Running a Cargo Project

```shell
cargo build
```

```shell
cargo run
```

```shell
cargo check
```

```shell
cargo build --release
```

#### Why would you not want an executable?

Oftern, `cargo check` is much faster than `cargo build`, because it skips the step of producing an executable. 

### Summary

* Install the latest stable version of Rust using `rustup`
* Update to a newer Rust version
* Open locally installed documentation
* Write and run a "Hello, world!" Program using `rustc` directly
* Create and run a new project using the conventions of Cargo