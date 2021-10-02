#![feature(rustc_private)]
extern crate rustc_driver;

fn main() {
    rustc_driver::RunCompiler::new(
        &std::env::args().collect::<Vec<_>>(),
        &mut lints::CompilerCallback,
    )
    .run()
    .unwrap();
}
