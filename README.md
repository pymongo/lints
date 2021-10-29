# custom rustc lints

## How to use these lint framework

1. add this crate as dependency on Cargo.toml
2. add `#![plugin(lints)]` to your crate root like <examples/compiler_plugin.rs>
3. cargo check # would apply lints from plugin crate

## How to run ui_test

> cargo test -- --nocapture

## ~~How to run(may not work or crate std not found)~~

in rust project directory you want to analyze

compile lints package rustc_ binary and set env RUSTC to rustc_ binary path, e.g. mac/linux:

> RUSTC=/path/to/rustc_ cargo check

---

## road map

- [ ] add f32 cast to f64 precession lost lint(check f32 cast to f64 and warn precession lost(clippy::pedantic has `f64 as f32` checking))
- [x] add ui_test similar to dylint::ui_test or rustc/clippy ui_test
- [ ] use a readable diff tool for ui test, better than dtolnay/dissimilar used by rust-analyzer
