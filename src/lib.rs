#![doc = include_str!("../README.md")]
#![feature(rustc_private, const_type_name)]
extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_span;

mod lints;

pub struct CompilerCallback;

impl rustc_driver::Callbacks for CompilerCallback {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.register_lints = Some(Box::new(move |_session, lint_store| {
            lints::register_early_pass_lints(lint_store);
        }));
    }
}
