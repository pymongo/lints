# custom rustc lints

## How to run

first need to install binary form source code:

> cargo install --path .

and then in rust project directory you want to analyze

mac/linux:

> RUSTC=lints cargo check

windows:

```
set RUSTC=lints
cargo check
set RUSTC=
```

---

## road map

- [ ] add f32 cast to f64 precession lost lint
- [ ] add ui_test similar to dylint::ui_test or rustc/clippy ui_test
