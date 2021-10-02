#![feature(rustc_private)]
extern crate rustc_driver;

fn main() {
    // std::env::set_var("RUSTC_LOG", "trace");
    // RUSTC_LOG=rustc_infer::infer::error_reporting=info rustc +my_rustc file.rs
    rustc_driver::init_rustc_env_logger();
    rustc_driver::RunCompiler::new(
        &std::env::args().collect::<Vec<_>>(),
        &mut lints::CompilerCallback,
    )
    .run()
    .unwrap();
}
