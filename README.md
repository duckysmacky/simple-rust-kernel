# Simple Rust Kernel

A simple OS kernel fully written in Rust.

## About

Since this was my first time working on a proper operating system, I was refering to the following [series of blog posts on building a Rust kernel from scratch](https://os.phil-opp.com/) in order to guide me on how such projects are developed. Allthough this project's source code closely matches the code provided in the blog, this kernel still features a lot of my own additions, changes and different choices in the development. In the future I might expand this project even further and add more custom features on top of an already built kernel.

Currently, this project utilizes a lot of dependencies in order to avoid re-writing common datastructures present on a x86_64 architecture, which greatly sped up development time.

## Prerequisites

- Rust Nightly
- QEMU

## Running

Thanks to the `bootloader` crate, we can easily start the kernel right on a QEMU virtual machine. This means that every cargo command will automatically build the kernel and start it in QEMU.

#### Starting the Kernel
```bash
cargo run
```

This will start the kernel right in QEMU. This is possible due to the `bootloader` crate providing a custom `bootimage runner`, which was configured to run in the `.cargo/config.toml` file on every `cargo run` command.

#### Testing the Kernel
```
cargo test
```

This runs all the integration tests defined in the `tests` directory, all the unit tests defined in the `crate` module (library files) and unit tests in the `main.rs` file. This project has a custom test runner which will run the tests in QEMU, display the results in the terminal and exit the VM correctly on failure.
