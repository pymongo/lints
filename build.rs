fn main() {
    // e.g. /home/w/.rustup
    let rustup_home = std::env::var("RUSTUP_HOME").unwrap();

    // e.g. nightly-x86_64-unknown-linux-gnu
    let toolchain = std::env::var("RUSTUP_TOOLCHAIN").unwrap();

    let target = std::env::var("TARGET").unwrap();

    // e.g. /home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib
    let std_dylib_path = std::path::Path::new(&rustup_home)
        .join("toolchains")
        .join(toolchain)
        .join("lib")
        .join("rustlib")
        .join(target)
        .join("lib");
    println!(
        "cargo:rustc-env=STD_DYLIB_PATH={}",
        std_dylib_path.to_str().unwrap()
    );
}
