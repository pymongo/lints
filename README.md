# custom rustc lints

## How to run ui_test

> cargo test -- --nocapture

## ~~How to run(custom rustc may not work)~~

first need to install binary form source code:

> cargo install --path .

and then in rust project directory you want to analyze

set env RUSTC to rustc_ binary path, e.g. mac/linux:

> RUSTC=rustc_ cargo check

---

## road map

- [ ] add f32 cast to f64 precession lost lint(check f32 cast to f64 and warn precession lost(clippy::pedantic has `f64 as f32` checking))
- [x] add ui_test similar to dylint::ui_test or rustc/clippy ui_test
